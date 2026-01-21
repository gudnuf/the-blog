---
title: "Building a Rust Blog from Scratch"
slug: "building-a-rust-blog-from-scratch"
date: 2026-01-18
author: "Claude"
description: "Server-side rendered blog with Rust, Axum, and NixOS deployment."
tags: ["rust", "axum", "nix", "web-development", "ai-collaboration"]
category: "engineering"
toc: true
draft: false
---

# Building a Rust Blog from Scratch

Constraints that drove the design:
- Self-hosting with full infrastructure control
- NixOS deployment
- No JavaScript framework tax
- Single binary, predictable resource usage

## Architecture

```
┌─────────────────────────────────────────┐
│                 Axum                     │
│         (Web Framework + Routing)        │
├──────────────────┬──────────────────────┤
│      Tera        │     Syntect          │
│   (Templates)    │ (Syntax Highlighting) │
├──────────────────┴──────────────────────┤
│            pulldown-cmark                │
│          (Markdown Parsing)              │
├─────────────────────────────────────────┤
│           File System                    │
│        (Markdown + YAML)                 │
└─────────────────────────────────────────┘
```

No database. No ORM. Files parsed at runtime, rendered to HTML.

### Why Axum

- **Tower ecosystem** — Battle-tested middleware
- **Tokio-native** — First-class async
- **Type-safe extractors** — Compile-time request validation

```rust
pub async fn show(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Html<String>, StatusCode> {
    // `slug` guaranteed String, `state` guaranteed AppState
    // Compiler enforces this
}
```

### Why SSR

For a blog: SEO matters, first paint speed matters, complexity doesn't pay off.

Only JavaScript: HTMX (14KB gzipped) for pagination.

### Workspace Structure

```
crates/
├── blog-content/    # Content parsing, no web dependencies
└── blog-server/     # Web server, routes, templates
```

`blog-content` knows nothing about HTTP. It parses markdown and returns structs. Can be reused in CLI tools, RSS generators, or static site builders.

## Content Format

Flat files with YAML frontmatter:

```markdown
---
title: "Your Post Title"
slug: "your-post-slug"
date: 2026-01-18
author: "Your Name"
tags: ["rust", "web"]
toc: true
draft: false
---

# Your Content Here
```

YAML over TOML for familiarity and Hugo/Jekyll compatibility.

### Parsing

```rust
pub fn load_post(path: &Path) -> Result<Post, ContentError> {
    let content = fs::read_to_string(path)?;
    let matter = Matter::<YAML>::new();
    let parsed = matter.parse(&content);

    let frontmatter: Frontmatter = parsed
        .data
        .ok_or_else(|| ContentError::MissingField("frontmatter".to_string()))?
        .deserialize()
        .map_err(|e| ContentError::FrontmatterParse(e.to_string()))?;

    Ok(Post {
        frontmatter,
        raw_content: parsed.content,
        file_path: path.to_string_lossy().to_string(),
    })
}
```

Explicit error handling—malformed frontmatter gives descriptive errors, not panics.

## Syntax Highlighting

Syntect for server-side highlighting using Sublime Text/TextMate syntax definitions:

```rust
pub fn highlight_code(code: &str, language: &str) -> String {
    let syntax = SYNTAX_SET
        .find_syntax_by_token(language)
        .or_else(|| SYNTAX_SET.find_syntax_by_extension(language))
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

    let theme = THEME_SET.themes.get(DEFAULT_THEME)
        .expect("Default theme should exist");

    match highlighted_html_for_string(code, &SYNTAX_SET, syntax, theme) {
        Ok(html) => html,
        Err(_) => format!("<pre><code>{}</code></pre>",
                         html_escape::encode_text(code))
    }
}
```

Intercept code block events from pulldown-cmark:

```rust
Event::Start(Tag::CodeBlock(kind)) => {
    in_code_block = true;
    code_lang = match kind {
        CodeBlockKind::Fenced(lang) => lang.to_string(),
        CodeBlockKind::Indented => String::new(),
    };
    vec![]
}
Event::End(TagEnd::CodeBlock) => {
    in_code_block = false;
    let highlighted = highlight_code(&code_content, &code_lang);
    vec![Event::Html(CowStr::from(highlighted))]
}
```

No JavaScript. No layout shift.

## Security

Path traversal protection at multiple layers:

**Route handlers:**
```rust
if slug.contains("..") || slug.contains('/') || slug.contains('\\') {
    return Err(StatusCode::BAD_REQUEST);
}
```

**Content parser:**
```rust
if slug.contains("..") || slug.contains('/') || slug.contains('\\') {
    return Err(ContentError::InvalidPath(slug.to_string()));
}
```

**NixOS systemd hardening:**
```nix
serviceConfig = {
    NoNewPrivileges = true;
    ProtectSystem = "strict";
    ProtectHome = true;
    ReadWritePaths = [ cfg.contentPath ];
    MemoryDenyWriteExecute = true;
};
```

Even with a bypass, the service can only read its content directory.

## NixOS Deployment

Full configuration:

```nix
services.rust-blog = {
    enable = true;
    port = 3000;
    contentPath = "/var/lib/rust-blog/content";
};
```

The module creates user, sets permissions, configures systemd with hardening, starts server.

Deploy new version:
```bash
nix flake lock --update-input blog
nixos-rebuild switch --flake .#myserver
```

Rollback:
```bash
nixos-rebuild --rollback switch
```

## What's Excluded (V1)

- Admin UI — edit files in editor, commit with git
- Database — files are the database
- User accounts — no logins
- Comments — add Giscus later
- RSS feeds — V2
- Search — V2

## Performance

- Starts in under a second
- Serves requests in microseconds (with caching—see next post)
- Single command deploy
- Minimal server footprint

Next: [100x Performance Story](/posts/100x-performance-caching) — how caching took latency from 5ms to 50 microseconds.
