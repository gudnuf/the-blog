# Rust SSR Blog

A fast, server-side rendered blog built with Rust, Axum, Tera templates, and Tailwind CSS. Designed for easy deployment with Nix/NixOS.

## Features

- 🦀 **Rust-powered**: Built with Axum for blazing-fast performance
- 📝 **Markdown content**: Write posts in Markdown with YAML frontmatter
- 🎨 **Tailwind CSS**: Modern, utility-first styling
- ⚡ **HTMX**: Progressive enhancement for dynamic interactions
- 🔍 **Syntax highlighting**: Beautiful code blocks using Syntect
- 📑 **Table of contents**: Auto-generated TOC for long posts
- 📦 **Nix packaging**: Reproducible builds and easy NixOS deployment
- 🔒 **Security hardening**: Systemd service with comprehensive security settings

## Quick Start

### Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled, OR
- [Rust](https://rustup.rs/) 1.70+ with Cargo

### Development with Nix

```bash
# Enter development shell
nix develop

# Build the project
cargo build

# Run the server with hot reload (in one terminal)
cargo watch -x run

# Watch Tailwind CSS (in another terminal)
./scripts/watch-tailwind.sh

# Visit http://localhost:3000
```

### Development without Nix

```bash
# Install dependencies
# - Rust (via rustup)
# - Tailwind CSS standalone CLI

# Build the project
cargo build

# Run the server
cargo run

# Build Tailwind CSS
tailwindcss -i ./static/css/input.css -o ./static/css/tailwind.css --watch
```

## Project Structure

```
.
├── crates/
│   ├── blog-server/       # Axum web server
│   └── blog-content/      # Content parsing library
├── content/               # Your blog content
│   ├── posts/            # Blog posts (YYYY-MM-DD-slug.md)
│   ├── pages/            # Static pages (about.md, etc.)
│   └── images/           # Image assets
├── templates/            # Tera HTML templates
├── static/               # Static assets (CSS, JS)
├── nixos/               # NixOS module
└── flake.nix            # Nix flake configuration
```

## Writing Content

### Creating a Blog Post

Create a file in `content/posts/` with the format `YYYY-MM-DD-slug.md`:

```markdown
---
title: "Your Post Title"
slug: "your-post-slug"
date: 2025-01-15
author: "Your Name"
description: "A brief description for SEO and previews"
tags: ["rust", "web", "tutorial"]
category: "programming"
toc: true
draft: false
---

# Your Content Here

Write your post content in Markdown...

## Code Blocks

\`\`\`rust
fn main() {
    println!("Syntax highlighting works!");
}
\`\`\`
```

### Frontmatter Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String | Yes | Post title |
| `slug` | String | Yes | URL-safe identifier |
| `date` | Date | Yes | Publication date (YYYY-MM-DD) |
| `author` | String | No | Author name |
| `description` | String | No | Short description for previews |
| `tags` | Array | No | List of tags |
| `category` | String | No | Post category |
| `template` | String | No | Template name (default: "post") |
| `draft` | Boolean | No | Draft status (default: false) |
| `toc` | Boolean | No | Enable table of contents (default: false) |
| `updated` | Date | No | Last update date |
| `featured_image` | String | No | Path to featured image |

### Creating a Static Page

Create a file in `content/pages/` with the name `slug.md`:

```markdown
---
title: "About"
slug: "about"
template: "page"
---

# About This Blog

Your content here...
```

## Configuration

The server can be configured via environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `BLOG_HOST` | `127.0.0.1` | Host to bind to |
| `BLOG_PORT` | `3000` | Port to listen on |
| `BLOG_CONTENT_PATH` | `./content` | Path to content directory |
| `BLOG_TEMPLATES_PATH` | `./templates` | Path to templates |
| `BLOG_STATIC_PATH` | `./static` | Path to static assets |
| `BLOG_POSTS_PER_PAGE` | `10` | Posts per page |
| `BLOG_ENABLE_DRAFTS` | `false` | Show draft posts |
| `RUST_LOG` | `info` | Logging level |

### Example

```bash
export BLOG_PORT=8080
export BLOG_ENABLE_DRAFTS=true
export RUST_LOG=debug
cargo run
```

## NixOS Deployment

### Using the NixOS Module

Add to your NixOS configuration:

```nix
{
  inputs.rust-blog.url = "github:yourusername/blog";

  outputs = { self, nixpkgs, rust-blog, ... }: {
    nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
      modules = [
        rust-blog.nixosModules.default
        {
          services.rust-blog = {
            enable = true;
            port = 3000;
            host = "0.0.0.0";
            contentPath = "/var/blog/content";
          };
        }
      ];
    };
  };
}
```

### Module Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enable` | Boolean | `false` | Enable the service |
| `host` | String | `"127.0.0.1"` | Host address |
| `port` | Port | `3000` | Port number |
| `contentPath` | Path | `/var/lib/rust-blog/content` | Content directory |
| `templatesPath` | Path | `null` | Templates (uses package default) |
| `staticPath` | Path | `null` | Static assets (uses package default) |
| `postsPerPage` | Integer | `10` | Posts per page |
| `enableDrafts` | Boolean | `false` | Show drafts |
| `user` | String | `"rust-blog"` | Service user |
| `group` | String | `"rust-blog"` | Service group |
| `logLevel` | String | `"info"` | Log level |

### Managing Content

Deploy your content to the server:

```bash
# Copy content to the server
rsync -avz content/ myhost:/var/lib/rust-blog/content/

# Restart the service
ssh myhost systemctl restart rust-blog
```

### Service Management

```bash
# Check status
systemctl status rust-blog

# View logs
journalctl -u rust-blog -f

# Restart service
systemctl restart rust-blog
```

## Building for Production

### With Nix

```bash
# Build the package
nix build

# The binary will be at
./result/bin/blog-server

# Test it
./result/bin/blog-server
```

### Without Nix

```bash
# Build release binary
cargo build --release

# Build production CSS
tailwindcss -i ./static/css/input.css -o ./static/css/tailwind.css --minify

# Run
./target/release/blog-server
```

## Development

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Check for issues
cargo check
```

### Hot Reload

Use `cargo-watch` for automatic rebuilding:

```bash
cargo watch -x run
```

## Architecture

### Crates

- **blog-content**: Library for parsing markdown files with frontmatter
  - Markdown parsing with `pulldown-cmark`
  - Syntax highlighting with `syntect`
  - Table of contents generation
  - Frontmatter parsing with `gray_matter`

- **blog-server**: Web server built with Axum
  - Route handlers
  - Template rendering with Tera
  - Static file serving
  - Configuration management

### Routes

| Route | Description |
|-------|-------------|
| `GET /` | Homepage with recent posts |
| `GET /posts` | All posts with pagination |
| `GET /posts/:slug` | Individual post |
| `GET /pages/:slug` | Static page |
| `GET /health` | Health check endpoint |
| `GET /static/*` | Static assets |
| `GET /images/*` | Content images |

## Customization

### Templates

Templates are in `templates/` and use Tera syntax:

- `base.html` - Base layout
- `index.html` - Homepage
- `post.html` - Blog post detail
- `page.html` - Static page
- `post_list.html` - Post listing
- `partials/` - Reusable components

### Styling

Tailwind CSS is configured in `tailwind.config.js`. Customize:

```javascript
module.exports = {
  theme: {
    extend: {
      colors: {
        // Add your colors
      },
    },
  },
  plugins: [],
}
```

## Performance

- Static file serving with compression
- Efficient markdown parsing
- Syntax highlighting cached at build time
- Minimal JavaScript (only HTMX)
- Production builds are optimized

## Security

The NixOS module includes hardening:

- Runs as unprivileged user
- Limited filesystem access
- No new privileges
- Memory execution protection
- Namespace restrictions

## License

Dual-licensed under MIT OR Apache-2.0

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## Support

- GitHub Issues: Report bugs or request features
- Documentation: See inline code documentation with `cargo doc --open`

## Acknowledgments

Built with:
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tera](https://github.com/Keats/tera) - Template engine
- [Syntect](https://github.com/trishume/syntect) - Syntax highlighting
- [Tailwind CSS](https://tailwindcss.com) - CSS framework
- [HTMX](https://htmx.org) - HTML interactions
- [Nix](https://nixos.org) - Package management
