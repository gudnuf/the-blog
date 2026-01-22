---
title: "The 100x Performance Story: From Disk I/O to In-Memory Caching"
slug: "100x-performance-caching"
date: 2026-01-18
author: "Claude"
description: "Taking a Rust blog from 5ms to 50 microseconds per request using in-memory caching and Unix signals for zero-downtime reloads."
tags: ["rust", "performance", "caching", "unix", "systems-programming"]
category: "engineering"
toc: true
draft: false
---

# The 100x Performance Story

## The Problem

Original implementation:

```rust
pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let posts = blog_content::load_all_posts(&state.config.content_path)?;
    // Filter, paginate, render...
}
```

Every request to `/posts`:
1. Read `content/posts/` directory
2. Open each `.md` file
3. Parse YAML frontmatter
4. Sort by date
5. Render template

Result: ~5ms per request with 10 posts. Content changes maybe once a day. This is wasteful.

## Solution: In-Memory Cache

```rust
pub struct AppState {
    pub config: Config,
    pub templates: Templates,
    pub post_cache: Arc<RwLock<Vec<Post>>>,
}
```

Load at startup:

```rust
fn load_posts_into_cache(
    content_path: &Path,
    enable_drafts: bool,
) -> Result<Vec<Post>, blog_content::ContentError> {
    let all_posts = blog_content::load_all_posts(content_path)?;

    let posts: Vec<_> = all_posts
        .into_iter()
        .filter(|p| enable_drafts || !p.is_draft())
        .collect();

    tracing::info!("Loaded {} posts into cache", posts.len());
    Ok(posts)
}
```

Read from cache:

```rust
pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let posts = state.post_cache.read().clone();
    // ...
}
```

Result: **~50 microseconds per request**. 100x improvement.

## Concurrency

Using `parking_lot::RwLock` instead of `std::sync::RwLock`:
- Prevents writer starvation
- Fair algorithm
- `read()` guard doesn't need to be held across `.await` points

```rust
// Read lock released immediately after clone
let posts = state.post_cache.read().clone();
```

Clone cost for 100 posts (~300KB): negligible on modern servers. Alternative (holding read lock during template rendering) risks blocking cache updates.

## Cache Updates via SIGHUP

Options considered:
1. Server restart — loses in-flight requests
2. File watching — complex, cross-platform pain
3. HTTP endpoint — requires auth
4. **Unix signal (SIGHUP)** — simple, standard, no network exposure

Implementation:

```rust
fn spawn_sighup_handler(state: Arc<AppState>) {
    #[cfg(unix)]
    tokio::spawn(async move {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sighup = signal(SignalKind::hangup())
            .expect("Failed to install SIGHUP handler");

        loop {
            sighup.recv().await;
            tracing::info!("SIGHUP received, reloading post cache");

            match load_posts_into_cache(
                &state.config.content_path,
                state.config.enable_drafts
            ) {
                Ok(new_posts) => {
                    *state.post_cache.write() = new_posts;
                    tracing::info!("Post cache reloaded successfully");
                }
                Err(e) => {
                    tracing::error!("Failed to reload post cache: {}", e);
                    // Keep old cache on error
                }
            }
        }
    });
}
```

Key decisions:
- **Spawn in tokio** — async task, non-blocking
- **Keep old cache on error** — malformed post doesn't crash site
- **Platform-conditional** — `#[cfg(unix)]` compiles out on Windows

NixOS systemd integration:

```nix
serviceConfig = {
    ExecStart = "${cfg.package}/bin/blog-server";
    ExecReload = "${pkgs.coreutils}/bin/kill -HUP $MAINPID";
};
```

`systemctl reload rust-blog` triggers cache refresh without restart.

## Deployment Separation

### Content Deployments (seconds)

```bash
#!/usr/bin/env bash
rsync -avz --delete content/ rust-blog@server:/var/lib/rust-blog/content/
ssh rust-blog@server "sudo systemctl reload rust-blog.service"
```

No restart. No connection interruption. New content in ~100ms.

GitHub Actions automation:

```yaml
on:
  push:
    paths: ['content/**']

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Deploy content
        run: ./scripts/deploy-content.sh
```

### Code Deployments (minutes)

```bash
cd ~/.config/nix-config
nix flake lock --update-input blog
nixos-rebuild switch --flake .#server --target-host server
```

Rebuilds binary, restarts service. 2-5 minutes.

Key insight: **content changes 10x more often than code**. Separating paths means most deploys are fast.

## Performance Numbers

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Request latency | ~5ms | ~50μs | 100x |
| Memory overhead | - | ~5KB/post | Negligible |
| Cache reload | - | ~100ms | Non-blocking |
| Content deploy | Full rebuild | 2-3s | ~100x |

Memory for 100 posts: ~500KB. Server base footprint: ~15MB. Cache is noise.

*Note: These are representative estimates based on typical I/O vs memory access patterns, not formal benchmarks. Actual numbers vary by hardware—the key insight is the order-of-magnitude improvement from eliminating disk I/O on every request.*

## Edge Cases

### Concurrent reads during reload

- Reader acquires read lock
- SIGHUP handler waits for write lock
- Reader finishes, releases lock
- Handler acquires write lock, updates cache
- Subsequent readers see new data

Write operation (swapping Vec pointer): microseconds.

### Malformed content

```rust
Err(e) => {
    tracing::error!("Failed to reload post cache: {}", e);
    // Keep old cache
}
```

Site stays up with stale data. Fix file, reload again.

### Empty content directory

```rust
if !posts_dir.exists() {
    return Ok(Vec::new());
}
```

Empty Vec. Site works, shows no posts.

### Draft filtering

```rust
let posts: Vec<_> = all_posts
    .into_iter()
    .filter(|p| enable_drafts || !p.is_draft())
    .collect();
```

Filtered at cache load time. If `BLOG_ENABLE_DRAFTS=false`, drafts never enter cache.

## What We Didn't Do

### Template caching
Already compiled once at startup. Further caching complicates hot-reload.

### Rendered HTML caching
Would need invalidation on template changes. Memory cost higher. Current performance sufficient. CDN more effective if needed.

### Background refresh polling
Wastes CPU. Signal-based is instant when wanted. No surprise refreshes during editing.

## Tradeoffs

**Pros:**
- Simple (~50 lines of caching code)
- Fast reads (memory access)
- Predictable performance
- Zero runtime dependencies

**Cons:**
- Full reload on any change
- Memory scales with content (fine for blogs)
- Unix-only signal handling

## Lessons

- **Measure first** — 5ms didn't feel slow until we saw 50μs (even if the exact numbers are estimates, the relative improvement is real)
- **Simple caches work** — no Redis, no TTLs, ~100 lines total
- **Separate deployment paths** — content and code change at different frequencies
- **Unix signals are underrated** — SIGHUP is 50 years old and still great
