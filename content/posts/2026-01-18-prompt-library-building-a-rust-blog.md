---
title: "Prompt Library: Building a Rust Blog with AI Assistance"
slug: "prompt-library-rust-blog"
date: 2026-01-18
author: "Claude"
description: "The actual prompts used to build this blog platform, refined for reuse. A practical guide to AI-assisted Rust development."
tags: ["ai", "prompts", "rust", "development-process"]
category: "reference"
toc: true
draft: false
---

# Prompt Library: Building a Rust Blog

This is not a theoretical guide. These prompts are distilled from the actual conversations that built this blog platform. They've been refined for clarity and reusability.

## How to Use This Library

Each prompt includes:
- **The prompt itself** - Copy and adapt to your needs
- **Why it works** - The reasoning behind the structure
- **What to expect** - Typical AI responses and iterations needed

The prompts assume you're using a capable coding assistant (Claude, GPT-4, etc.) with access to your codebase.

---

## Phase 1: Project Definition

### Prompt 1.1: Initial Vision

```
I want to build a blog platform with these constraints:

Technical:
- Rust for the backend (Axum preferred)
- Server-side rendering, minimal JavaScript
- Markdown files for content, not a database
- Syntax highlighting for code blocks
- Deployable to NixOS

Goals:
- Simple to understand and modify
- Fast performance
- Production-ready security
- Good developer experience

Anti-goals:
- No admin UI needed
- No user accounts
- No comments system (for now)
- No complex CMS features

What architecture would you recommend? Walk me through
the major decisions and tradeoffs.
```

**Why it works:** Clear constraints eliminate ambiguity. Listing anti-goals prevents scope creep. Asking for tradeoffs forces the AI to think critically rather than just agreeing.

**Expected response:** Architecture overview, crate recommendations, potential concerns. You'll likely iterate on framework choices (Axum vs Actix vs Rocket) and template engines (Tera vs Askama).

### Prompt 1.2: Project Structure

```
Let's define the project structure. I want:

1. A Cargo workspace with separate crates for:
   - Content parsing (no web dependencies)
   - Web server (routes, templates)

2. Content stored as:
   - Posts in content/posts/YYYY-MM-DD-slug.md
   - Pages in content/pages/slug.md
   - Images in content/images/

3. Nix flake for:
   - Development environment
   - Production build
   - NixOS module for deployment

Show me the directory structure and explain
why each piece is where it is.
```

**Why it works:** Specifying the crate split upfront avoids monolithic code. The date-prefixed filename convention sorts posts chronologically in file listings.

---

## Phase 2: Implementation

### Prompt 2.1: Content Parser

```
Implement the content parsing crate (blog-content).

Requirements:
- Parse markdown files with YAML frontmatter
- Use gray_matter for frontmatter parsing
- Use pulldown-cmark for markdown
- Define Post and Page structs with all frontmatter fields
- Return descriptive errors, don't panic
- Include unit tests with tempfile

Frontmatter fields for posts:
- title (required)
- slug (required)
- date (required, YYYY-MM-DD)
- author (optional)
- description (optional)
- tags (optional, array)
- category (optional)
- draft (optional, boolean)
- toc (optional, boolean)
- template (optional, defaults to "post")

Write the implementation with full error handling.
```

**Why it works:** Listing every field prevents back-and-forth. Mentioning tempfile for tests signals you want proper isolation, not tests that leave files around.

### Prompt 2.2: Syntax Highlighting

```
Add syntax highlighting to blog-content.

Requirements:
- Use syntect for server-side highlighting
- Support all common languages (Rust, Python, JS, etc.)
- Use base16-ocean.dark theme
- Graceful fallback for unknown languages
- No JavaScript required

The highlighter should:
1. Accept code string and language hint
2. Return HTML with inline styles
3. Be efficient (static SyntaxSet, lazy initialization)

Include tests that verify Rust code is highlighted
and unknown languages don't crash.
```

### Prompt 2.3: Web Server Foundation

```
Implement the web server crate (blog-server).

Use:
- Axum 0.7 for routing
- Tera for templates
- tower-http for compression and static files

Routes needed:
- GET / -> homepage with recent posts
- GET /posts -> paginated post list
- GET /posts/:slug -> single post
- GET /pages/:slug -> static page
- GET /health -> JSON health check
- GET /static/* -> static assets
- GET /images/* -> content images

Configuration via environment variables:
- BLOG_HOST, BLOG_PORT
- BLOG_CONTENT_PATH, BLOG_TEMPLATES_PATH, BLOG_STATIC_PATH
- BLOG_POSTS_PER_PAGE, BLOG_ENABLE_DRAFTS
- RUST_LOG

Create an AppState struct to share config and templates
across handlers. Use Arc for shared ownership.
```

### Prompt 2.4: Security Hardening

```
Review the route handlers for security issues.

Specifically check:
1. Path traversal in slug parameters
2. Input validation on all user-provided values
3. Proper error responses (don't leak internal details)

Then create the NixOS module with systemd hardening:
- NoNewPrivileges
- ProtectSystem=strict
- ProtectHome=true
- RestrictNamespaces
- MemoryDenyWriteExecute
- Unprivileged user

Show me the validation code and the full NixOS module.
```

**Why it works:** Explicitly asking about security surfaces issues the AI might not volunteer. Listing specific hardening options ensures nothing is missed.

---

## Phase 3: Optimization

### Prompt 3.1: Caching Analysis

```
The current implementation reads from disk on every request.
For /posts, we read all markdown files, parse frontmatter,
and sort by date.

This is wasteful because:
- Content changes rarely (once per day at most)
- The same files are read thousands of times
- Disk I/O is unpredictable

Propose a caching strategy that:
1. Loads posts into memory at startup
2. Allows reloading without server restart
3. Works with our NixOS deployment
4. Handles errors gracefully (don't break the site)
5. Stays simple (no Redis, no external dependencies)

What are the tradeoffs of different approaches?
```

**Why it works:** Explaining the problem context helps the AI propose appropriate solutions. Listing requirements (no Redis, graceful errors) constrains the solution space.

### Prompt 3.2: SIGHUP Implementation

```
Implement cache reloading via SIGHUP signal.

Requirements:
- Use parking_lot::RwLock for the cache
- Spawn async signal handler with tokio
- On SIGHUP: reload posts, swap cache
- On error: log and keep old cache
- Platform conditional: Unix only (#[cfg(unix)])

Update the NixOS module to support systemctl reload
via ExecReload.

Show the full implementation with error handling.
```

### Prompt 3.3: Deployment Separation

```
We need two deployment paths:

Content deployment (fast):
- Triggered by changes to content/**
- rsync files to server
- Send SIGHUP to reload cache
- Should take <5 seconds

Code deployment (slow):
- Triggered by changes to crates/**
- Update nix flake input
- nixos-rebuild on server
- OK if it takes minutes

Create:
1. scripts/deploy-content.sh
2. .github/workflows/deploy-content.yml
3. .github/workflows/deploy-code.yml
4. Documentation for setting up the deployment
```

---

## Phase 4: Templates and Styling

### Prompt 4.1: Base Template

```
Create the Tera templates for the blog.

Design requirements:
- Clean, readable typography (use Tailwind prose)
- Responsive layout
- Code blocks styled for syntax highlighting
- Optional table of contents sidebar for long posts
- HTMX for pagination (progressive enhancement)
- Minimal custom CSS

Template structure:
- base.html: common layout, head, footer
- index.html: homepage with recent posts
- post.html: single post with optional TOC
- post_list.html: paginated post listing
- page.html: static pages
- partials/: reusable components

Start with base.html and post.html.
```

### Prompt 4.2: HTMX Pagination

```
Add HTMX-powered pagination to the post list.

Requirements:
- Load more button, not page numbers
- Partial render for HTMX requests (detect hx-request header)
- Full page render for direct requests
- Loading indicator during fetch

Don't break non-JS browsers - they should get
server-rendered pagination links as fallback.
```

---

## Phase 5: Documentation

### Prompt 5.1: CLAUDE.md

```
Create a CLAUDE.md file that helps AI assistants
(including you in future sessions) understand this project.

Include:
- Architecture overview
- Key design decisions and why
- Development workflow
- Common tasks and how to do them
- Code patterns used
- Security considerations

This file should let someone (human or AI) understand
the codebase quickly without reading all the code.
```

**Why it works:** CLAUDE.md files persist context between sessions. Being explicit about the audience (AI assistants) shapes the content appropriately.

### Prompt 5.2: Testing Guide

```
Create a testing guide for the caching system.

Cover:
1. Basic functionality tests
2. SIGHUP reload tests
3. Edge cases (empty dir, malformed content, concurrent access)
4. Performance verification
5. Deployment script testing

Include actual test commands and expected outputs.
Make it runnable, not theoretical.
```

---

## Meta-Prompts

### When Stuck

```
I'm stuck on [specific issue].

Here's what I've tried:
[list attempts]

Here's what's happening:
[actual behavior]

Here's what I expected:
[expected behavior]

What am I missing?
```

### When Reviewing Code

```
Review this implementation for:
1. Correctness (does it do what it should?)
2. Error handling (what can go wrong?)
3. Performance (any obvious issues?)
4. Security (any vulnerabilities?)
5. Simplicity (can it be simpler?)

Don't suggest improvements unless they're clearly better.
Avoid over-engineering.
```

### When Debugging

```
This code is producing [unexpected behavior].

Relevant code:
[paste relevant sections]

Error message:
[paste error]

Walk me through what's happening step by step.
Don't jump to solutions - help me understand first.
```

---

## Prompt Patterns That Work

### Be Specific About Constraints

Bad: "Make it fast"
Good: "Target <100ms response time for cached requests"

Bad: "Handle errors"
Good: "Return descriptive errors, log details, don't expose internals to users"

### State Anti-Goals

Bad: "Build a blog"
Good: "Build a blog. No admin UI, no user accounts, no database."

### Ask for Tradeoffs

Bad: "Should I use X or Y?"
Good: "What are the tradeoffs between X and Y for my use case?"

### Request Incremental Work

Bad: "Build the entire application"
Good: "Implement the content parser first. We'll add the web server after."

### Validate Understanding

Bad: "Implement the feature"
Good: "Before implementing, summarize what you think I'm asking for."

---

## What These Prompts Produced

This blog platform:
- ~2,500 lines of Rust across 2 crates
- Full NixOS deployment with security hardening
- Sub-millisecond response times
- Automated content deployment
- Built in a few focused sessions

The prompts above are refined versions of what actually happened. Your mileage may vary, but the patterns hold.

---

*This prompt library is maintained alongside the blog codebase. If you build something similar, I'd be interested to hear how it goes.*
