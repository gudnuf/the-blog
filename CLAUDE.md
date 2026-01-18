# Rust SSR Blog - Project Context

Server-side rendered blog built with Axum, Tera templates, and Tailwind CSS. Deployable via NixOS module.

## Architecture

**Workspace Structure**: Two crates in cargo workspace
- `blog-server`: Axum web server with route handlers, templates, config
- `blog-content`: Library for markdown parsing, syntax highlighting, TOC generation

**Key Dependencies**:
- Web: axum 0.7, tokio, tower-http (compression, static serving)
- Templates: tera 1.19
- Markdown: pulldown-cmark 0.11, gray_matter 0.2 (frontmatter)
- Highlighting: syntect 5.2
- Data: serde, serde_yaml, chrono

## Development Workflow

```bash
# Enter dev shell (provides rust, cargo-watch, tailwindcss, bacon)
nix develop

# Run with hot reload
cargo watch -x run

# Watch Tailwind CSS (separate terminal)
./scripts/watch-tailwind.sh

# Server runs on http://localhost:3000
```

**Without Nix**: Requires Rust 1.70+ and Tailwind CLI installed separately.

## Content Model

**Posts** (`content/posts/YYYY-MM-DD-slug.md`):
- YAML frontmatter with required fields: `title`, `slug`, `date`
- Optional: `author`, `description`, `tags`, `category`, `draft`, `toc`, `template`, `updated`, `featured_image`, `related_posts`
- Sorted by date (newest first), filtered by draft status

**Pages** (`content/pages/slug.md`):
- Simpler frontmatter: `title`, `slug`, `template` (defaults to "page")
- Loaded on-demand by slug

**Parsing**: `blog-content/parser.rs` uses gray_matter for frontmatter, returns `Post`/`Page` structs.

## Routing

```
GET /                    -> index (recent posts)
GET /posts               -> post list with pagination
GET /posts/page/:page    -> HTMX partial for pagination
GET /posts/:slug         -> single post
GET /pages/:slug         -> static page
GET /health              -> health check
GET /static/*            -> static assets
GET /images/*            -> content/images
```

**Route Pattern**: State(Arc<AppState>) contains Config, Templates, and post cache. All handlers return `Result<Html<String>, StatusCode>`.

## Rendering Pipeline

1. **Load**: Parser reads markdown file, extracts frontmatter
2. **TOC**: If `toc: true`, extract headings from markdown (`blog-content/toc.rs`)
3. **Markdown**: Parse with pulldown-cmark, intercept code blocks
4. **Highlighting**: Syntect highlights code blocks (`blog-content/highlighter.rs`)
5. **Template**: Tera renders HTML with context (post/page data, content HTML, TOC)

**Code Highlighting**: Custom event processing in `posts.rs:render_markdown_with_highlighting()` intercepts `CodeBlock` events, replaces with highlighted HTML.

**Heading IDs**: Auto-generated for TOC anchor links during markdown parsing.

## Configuration

Env-based with defaults (see `blog-server/config.rs`):
- `BLOG_HOST` (127.0.0.1), `BLOG_PORT` (3000)
- `BLOG_CONTENT_PATH` (./content)
- `BLOG_TEMPLATES_PATH` (./templates)
- `BLOG_STATIC_PATH` (./static)
- `BLOG_POSTS_PER_PAGE` (10)
- `BLOG_ENABLE_DRAFTS` (false)
- `RUST_LOG` (info)

**Path Validation**: Config validates paths exist on load, creates content/posts/pages dirs if missing.

## Caching

**Post Cache**: All posts loaded into `Arc<RwLock<Vec<Post>>>` at startup. Pre-filtered for draft status.

**SIGHUP Reload**: Send `SIGHUP` to reload cache without restart:
```bash
systemctl reload rust-blog  # or: kill -HUP $(pidof blog-server)
```

**Dependency**: Uses `parking_lot::RwLock` for efficient concurrent reads.

## Templates

Tera syntax, base layout pattern:
- `base.html`: Base layout with nav, footer
- `index.html`: Homepage (recent posts)
- `post.html`: Single post with optional TOC sidebar
- `page.html`: Static page
- `post_list.html`: Post listing with pagination
- `partials/`: Header, nav, footer, post_list_items (HTMX)

**HTMX**: Pagination uses `hx-get` for post_list_items partial.

## Security

**Path Traversal Protection**: Slug validation in `posts.rs` and `pages.rs` blocks `..`, `/`, `\` characters.

**NixOS Hardening** (see `nixos/module.nix`):
- Unprivileged user, restricted filesystem access
- NoNewPrivileges, MemoryDenyWriteExecute
- ProtectSystem=strict, ProtectHome=true
- Namespace isolation (PID, IPC, UTS, etc.)

## Build & Deploy

**Nix Build**: `nix build` produces binary at `./result/bin/blog-server` with bundled templates/static/content.

**NixOS Module**: Import `nixosModules.default`, configure via `services.rust-blog.*` options.

**Production**: Binary expects content at configured paths. Use rsync to deploy content, systemctl to manage service.

**Scripts**: `scripts/deploy-content.sh` rsyncs content and triggers SIGHUP reload.

## Code Patterns

**Error Handling**:
- Content parsing: `ContentError` thiserror enum
- HTTP handlers: Map errors to StatusCode, log details

**Shared State**: `Arc<AppState>` with Config + Templates, passed to all handlers.

**Tests**: Unit tests in `parser.rs` use tempfile for isolated filesystem tests.

**Logging**: tracing with EnvFilter, includes tower-http request tracing.

## Common Tasks

**Add Route**: Define handler in `routes/*.rs`, register in `main.rs` router.

**New Template Variable**: Add to tera Context in handler, use in template with `{{ variable }}`.

**Content Fields**: Add to Frontmatter/PageFrontmatter struct, update templates.

**Styling**: Edit `static/css/input.css`, rebuild with Tailwind (watch script monitors changes).
