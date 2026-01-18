# Quick Start Guide

Get your blog running in 5 minutes!

## Option 1: Run with Cargo (Recommended for Development)

### Prerequisites
- Rust installed (get it from https://rustup.rs/)

### Steps

1. **Navigate to the project directory**:
   ```bash
   cd /Users/claude/blog
   ```

2. **Build and run the server**:
   ```bash
   cargo run
   ```

3. **Open your browser**:
   ```
   http://localhost:3000
   ```

That's it! You should see the blog homepage with the sample posts.

### Available Routes

- **Homepage**: http://localhost:3000
- **All Posts**: http://localhost:3000/posts
- **Sample Post**: http://localhost:3000/posts/getting-started-with-rust
- **About Page**: http://localhost:3000/pages/about
- **Health Check**: http://localhost:3000/health

## Option 2: Run with Hot Reload (Best for Development)

Install cargo-watch:
```bash
cargo install cargo-watch
```

Then run with auto-reload:
```bash
cargo watch -x run
```

Now any changes to Rust code will automatically rebuild and restart the server!

## Option 3: Run with Nix

If you have Nix with flakes enabled:

```bash
# Enter development shell
nix develop

# Run the server
cargo run
```

## Customizing Your Blog

### 1. Add a New Post

Create a file in `content/posts/`:

```bash
cat > content/posts/2025-01-17-my-first-post.md << 'EOF'
---
title: "My First Blog Post"
slug: "my-first-post"
date: 2025-01-17
author: "Your Name"
description: "This is my first post!"
tags: ["blog", "first-post"]
category: "meta"
toc: false
draft: false
---

# Hello World!

This is my first blog post. Welcome to my blog!

## What I'll Write About

- Technology
- Programming
- Life experiences

Stay tuned for more!
EOF
```

Refresh your browser and you'll see the new post!

### 2. Edit the About Page

Edit `content/pages/about.md` with your information.

### 3. Customize Templates

Edit files in `templates/` to change the look and feel:
- `templates/base.html` - Main layout
- `templates/index.html` - Homepage
- `templates/post.html` - Blog post layout
- `templates/partials/header.html` - Navigation

## Configuration

Set environment variables to customize behavior:

```bash
# Change port
export BLOG_PORT=8080

# Enable draft posts
export BLOG_ENABLE_DRAFTS=true

# Run the server
cargo run
```

## Building for Production

```bash
# Build optimized binary
cargo build --release

# Run it
./target/release/blog-server
```

## Troubleshooting

### Port Already in Use

If port 3000 is busy, change it:
```bash
export BLOG_PORT=8080
cargo run
```

### Missing Rust

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build Errors

Make sure you're in the project directory and dependencies are up to date:
```bash
cd /Users/claude/blog
cargo clean
cargo build
```

## Next Steps

- Read the full [README.md](README.md) for detailed documentation
- Check [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines
- Explore the code in `crates/blog-server` and `crates/blog-content`
- Customize templates and styles to make it your own!

## Need Help?

- Check the logs for errors
- Review the example posts in `content/posts/`
- Open an issue on GitHub
