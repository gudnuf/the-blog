# Local Testing Guide - Post Caching System

This guide covers testing the post caching system locally before deploying to production.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Basic Functionality Tests](#basic-functionality-tests)
- [Cache Performance Tests](#cache-performance-tests)
- [SIGHUP Reload Tests](#sighup-reload-tests)
- [Edge Cases](#edge-cases)
- [Deployment Script Tests](#deployment-script-tests)
- [Troubleshooting Local Tests](#troubleshooting-local-tests)

---

## Prerequisites

**Required:**
- Rust 1.70+ installed
- Content directory with sample posts

**Optional (for full testing):**
- Nix development shell
- `cargo-watch` for hot reload
- `hey` or `wrk` for load testing

**Enter development environment:**

```bash
# With Nix
nix develop

# Without Nix
# Ensure you have Rust and Cargo installed
cargo --version
```

---

## Basic Functionality Tests

### 1. Build and Run Tests

Verify the code compiles and passes tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check compilation
cargo check

# Build in release mode
cargo build --release
```

### 2. Start the Server

Run the development server:

```bash
# Standard run
cargo run

# With debug logging
RUST_LOG=debug cargo run

# Expected output:
# INFO  Configuration loaded
# INFO  Templates loaded from "./templates"
# INFO  Loaded 3 posts into cache  # ← Verify this line
# INFO  Starting server on http://127.0.0.1:3000
```

**Verify cache initialization:**
- Look for "Loaded N posts into cache" in logs
- Number should match posts in `content/posts/`

### 3. Test Basic Endpoints

In a new terminal, test all endpoints:

```bash
# Health check
curl http://localhost:3000/health
# Expected: {"status":"healthy"}

# Index page (should use cache)
curl http://localhost:3000/
# Expected: HTML with recent posts

# All posts page
curl http://localhost:3000/posts
# Expected: HTML with paginated post list

# Single post (should use cache)
curl http://localhost:3000/posts/getting-started
# Expected: HTML with post content or 404 if not exists

# Static assets
curl -I http://localhost:3000/static/css/output.css
# Expected: 200 OK
```

### 4. Verify Cache is Being Used

Check that routes are using the cache (not disk I/O):

```bash
# Watch logs while making requests
RUST_LOG=debug cargo run

# In another terminal, make requests
for i in {1..5}; do
  curl -s http://localhost:3000/posts >/dev/null
  echo "Request $i completed"
done

# In server logs, you should NOT see:
# - "Failed to load posts" repeatedly
# - Multiple file system reads for the same request
# - Slow response times

# You SHOULD see fast responses with no file I/O
```

---

## Cache Performance Tests

### 1. Manual Performance Comparison

Create a simple test to compare performance:

**Before cache (simulate by checking logs):**
```bash
# Check git history or temporarily revert changes
# Run old version and note response times

# With cache:
time curl -s http://localhost:3000/ > /dev/null
# Should be <50ms for full page render
```

### 2. Load Testing with `hey`

Install and run load tests:

```bash
# Install hey (if not using Nix)
# macOS: brew install hey
# Linux: go install github.com/rakyll/hey@latest

# Run 1000 requests with 10 concurrent connections
hey -n 1000 -c 10 http://localhost:3000/

# Expected results with cache:
# - Requests/sec: >500
# - Average latency: <20ms
# - No errors

# Test specific endpoints
hey -n 500 -c 5 http://localhost:3000/posts
hey -n 500 -c 5 http://localhost:3000/posts/your-post-slug
```

### 3. Memory Usage Testing

Monitor memory while running:

```bash
# Terminal 1: Start server
cargo run

# Terminal 2: Monitor memory
watch -n 1 'ps aux | grep blog-server | grep -v grep'

# Terminal 3: Generate load
hey -n 10000 -c 50 http://localhost:3000/

# Memory should remain stable:
# - Base memory: ~10-20 MB
# - With cache (100 posts): +0.5-1 MB
# - Should NOT grow continuously (would indicate leak)
```

---

## SIGHUP Reload Tests

### Test 1: Basic SIGHUP Reload

Verify the cache reloads when SIGHUP is sent:

```bash
# Terminal 1: Start server with debug logging
RUST_LOG=debug cargo run

# Expected in logs:
# INFO  Loaded 3 posts into cache
# INFO  Starting server on http://127.0.0.1:3000
```

```bash
# Terminal 2: Get server PID and send SIGHUP
pgrep blog-server
# Output: 12345 (example PID)

pkill -HUP blog-server
# Or: kill -HUP 12345
```

**Expected in server logs (Terminal 1):**
```
INFO  SIGHUP received, reloading post cache
INFO  Loaded 3 posts into cache
INFO  Post cache reloaded successfully
```

**Verify no errors:**
- Server should NOT crash
- Should NOT see "Failed to reload post cache"
- Server should still respond to requests

### Test 2: Reload with New Content

Test that new posts appear after SIGHUP:

```bash
# Terminal 1: Server running
cargo run
# Note: "Loaded 3 posts into cache"
```

```bash
# Terminal 2: Create new post
cat > content/posts/2026-01-17-test-reload.md <<EOF
---
title: "Test SIGHUP Reload"
slug: "test-reload"
date: 2026-01-17
author: "Test Author"
---

This post tests SIGHUP reload functionality.

# Test Heading

- Test bullet 1
- Test bullet 2
EOF

# Send SIGHUP
pkill -HUP blog-server

# Verify new post appears
curl http://localhost:3000/posts/test-reload
# Expected: HTML with post content

# Check post appears in list
curl http://localhost:3000/posts | grep "Test SIGHUP Reload"
# Expected: Should find the title
```

**Check server logs:**
```
INFO  SIGHUP received, reloading post cache
INFO  Loaded 4 posts into cache  # ← Should be 4 now (was 3)
INFO  Post cache reloaded successfully
```

### Test 3: Reload with Modified Content

Test that post changes are picked up:

```bash
# Terminal 1: Server running
cargo run
```

```bash
# Terminal 2: View existing post
curl http://localhost:3000/posts/test-reload | grep "Test SIGHUP Reload"

# Modify the post
cat > content/posts/2026-01-17-test-reload.md <<EOF
---
title: "Test SIGHUP Reload - UPDATED"
slug: "test-reload"
date: 2026-01-17
author: "Test Author"
---

This content has been UPDATED.
EOF

# Send SIGHUP
pkill -HUP blog-server

# Verify changes appear
curl http://localhost:3000/posts/test-reload | grep "UPDATED"
# Expected: Should find "UPDATED" in title and content
```

### Test 4: Reload with Deleted Post

Test that deleted posts are removed from cache:

```bash
# Terminal 1: Server running
cargo run
# Note current post count in logs

# Terminal 2: Delete a post
rm content/posts/2026-01-17-test-reload.md

# Send SIGHUP
pkill -HUP blog-server

# Try to access deleted post
curl -I http://localhost:3000/posts/test-reload
# Expected: HTTP/1.1 404 Not Found

# Check post count in logs (should be 1 less)
# INFO  Loaded 3 posts into cache  # Back to 3
```

### Test 5: Reload with Invalid Content

Test that cache reload handles errors gracefully:

```bash
# Terminal 1: Server running
cargo run
```

```bash
# Terminal 2: Create post with invalid frontmatter
cat > content/posts/2026-01-17-invalid.md <<EOF
---
title: "Missing closing frontmatter
slug: "invalid"
---

This will fail to parse.
EOF

# Send SIGHUP
pkill -HUP blog-server
```

**Expected in server logs:**
```
INFO  SIGHUP received, reloading post cache
ERROR Failed to reload post cache: [error details]
```

**Important: Server should:**
- NOT crash
- Keep old cache (still serving previous posts)
- Continue responding to requests

```bash
# Verify server still works with old cache
curl http://localhost:3000/posts
# Expected: Still returns posts

# Fix the invalid post
rm content/posts/2026-01-17-invalid.md

# Reload again
pkill -HUP blog-server
# Expected: Should succeed now
```

### Test 6: Draft Post Filtering

Test that drafts are filtered correctly in cache:

```bash
# Terminal 1: Server running (default: drafts disabled)
cargo run
```

```bash
# Terminal 2: Create draft post
cat > content/posts/2026-01-17-draft.md <<EOF
---
title: "Draft Post"
slug: "draft-post"
date: 2026-01-17
draft: true
---

This is a draft post.
EOF

# Reload cache
pkill -HUP blog-server

# Try to access draft
curl -I http://localhost:3000/posts/draft-post
# Expected: HTTP/1.1 404 Not Found

# Check it doesn't appear in list
curl http://localhost:3000/posts | grep "Draft Post"
# Expected: No match found
```

**Test with drafts enabled:**

```bash
# Terminal 1: Restart server with drafts enabled
BLOG_ENABLE_DRAFTS=true cargo run
# Should see draft included in count

# Terminal 2: Access draft
curl http://localhost:3000/posts/draft-post
# Expected: 200 OK, post content returned
```

---

## Edge Cases

### Test 1: Empty Content Directory

```bash
# Backup existing content
mv content/posts content/posts.bak
mkdir -p content/posts

# Start server
cargo run

# Expected in logs:
# INFO  Loaded 0 posts into cache

# Test endpoints
curl http://localhost:3000/
# Expected: Empty post list or "no posts" message

curl -I http://localhost:3000/posts
# Expected: 200 OK but empty list

# Restore content
rm -rf content/posts
mv content/posts.bak content/posts
```

### Test 2: Large Number of Posts

Test performance with many posts:

```bash
# Generate 1000 test posts
for i in {1..1000}; do
  cat > "content/posts/2026-01-$(printf "%02d" $((i % 28 + 1)))-test-$i.md" <<EOF
---
title: "Test Post $i"
slug: "test-post-$i"
date: 2026-01-$(printf "%02d" $((i % 28 + 1)))
---

This is test post number $i.
EOF
done

# Start server and measure load time
time cargo run &
SERVER_PID=$!

# Wait for server to start
sleep 3

# Check logs for cache load time
# Should see: "Loaded 1000 posts into cache"

# Test performance
time curl -s http://localhost:3000/ > /dev/null
# Should still be fast (<100ms)

# Send SIGHUP and measure reload time
time pkill -HUP blog-server
sleep 1
# Check logs for reload time (should be <1 second)

# Cleanup
kill $SERVER_PID
rm content/posts/2026-*-test-*.md
```

### Test 3: Concurrent Requests During Reload

Test that requests work during cache reload:

```bash
# Terminal 1: Server running
cargo run
```

```bash
# Terminal 2: Script to send requests while reloading
cat > test_concurrent.sh <<'EOF'
#!/bin/bash
# Send 100 requests in background
for i in {1..100}; do
  curl -s http://localhost:3000/posts > /dev/null &
done

# Send SIGHUP in the middle
sleep 0.1
pkill -HUP blog-server

# Wait for all requests to complete
wait
echo "All requests completed"
EOF

chmod +x test_concurrent.sh
./test_concurrent.sh
```

**Expected:**
- All requests should succeed (no errors)
- No race conditions or crashes
- Server logs may show reload happening during requests

### Test 4: Special Characters in Content

```bash
# Create post with special characters
cat > content/posts/2026-01-17-special.md <<'EOF'
---
title: "Special Characters: <>&\"'`"
slug: "special-chars"
date: 2026-01-17
---

Testing special characters:
- HTML: <script>alert('xss')</script>
- Quotes: "double" and 'single'
- Code: `backticks`
- Unicode: 你好 🚀 ñ
EOF

# Reload cache
pkill -HUP blog-server

# Test post renders correctly
curl http://localhost:3000/posts/special-chars

# Verify XSS protection (HTML should be escaped)
curl http://localhost:3000/posts/special-chars | grep "&lt;script&gt;"
# Expected: Should find escaped HTML

# Cleanup
rm content/posts/2026-01-17-special.md
```

---

## Deployment Script Tests

### Test Content Deployment Script Locally

Test the deployment script against a local target:

```bash
# Create a test "server" directory
mkdir -p /tmp/test-blog-server/content

# Set environment variables for test
export DEPLOY_HOST="localhost"
export DEPLOY_USER="$USER"
export DEPLOY_CONTENT_PATH="/tmp/test-blog-server/content"

# Run deployment script
./scripts/deploy-content.sh
```

**Expected output:**
```
==> Deploying content to localhost
==> Testing SSH connection
==> Syncing content
[rsync output]
==> Reloading post cache
[Will fail if service not running - this is OK for local test]
==> Deployment complete!
```

**Verify sync worked:**
```bash
ls -la /tmp/test-blog-server/content/
# Should see posts/, pages/, images/ directories

# Check content synced
diff -r content/ /tmp/test-blog-server/content/
# Should show no differences (or only .DS_Store exclusions)

# Cleanup
rm -rf /tmp/test-blog-server
```

---

## Troubleshooting Local Tests

### Server Won't Start

**Issue: Port already in use**
```
Error: Address already in use (os error 48)
```

**Solution:**
```bash
# Find process using port 3000
lsof -i :3000

# Kill it
kill -9 <PID>

# Or change port
BLOG_PORT=3001 cargo run
```

### SIGHUP Not Working

**Issue: No response to SIGHUP**

**macOS/Unix check:**
```bash
# Verify signal support
cargo run &
PID=$!
kill -HUP $PID
# Check logs for "SIGHUP received"
```

**Non-Unix systems:**
```
WARN  SIGHUP handler not available on non-Unix systems
```
- This is expected on Windows
- Use `systemctl reload` in production

### Cache Not Loading Posts

**Issue: "Loaded 0 posts into cache"**

**Check content path:**
```bash
ls -la content/posts/
# Should see .md files

# Run with debug logging
RUST_LOG=debug cargo run
# Look for errors about missing directories or parse failures
```

**Verify frontmatter:**
```bash
# Check first post
head -n 10 content/posts/*.md

# Required fields:
# - title
# - slug
# - date (YYYY-MM-DD format)
```

### Performance Issues

**Issue: Slow response times**

**Profile the application:**
```bash
# Build with release optimizations
cargo build --release

# Run release build
./target/release/blog-server

# Test performance
time curl -s http://localhost:3000/ > /dev/null

# If still slow:
# 1. Check template complexity
# 2. Verify using cache (not hitting disk)
# 3. Profile with flamegraph
```

### Memory Leak Detection

**Issue: Memory grows over time**

**Test for leaks:**
```bash
# Install valgrind (Linux)
# Or use instruments (macOS)

# Run with memory profiler
cargo build
valgrind --leak-check=full ./target/debug/blog-server

# Or use heaptrack (easier)
heaptrack ./target/debug/blog-server

# Let it run, send requests, send SIGHUP multiple times
# Check for growing allocations
```

---

## Test Checklist

Before deploying to production, verify:

- [ ] Server starts successfully with cache loaded
- [ ] All endpoints return expected responses
- [ ] SIGHUP reload works and updates cache
- [ ] New posts appear after reload
- [ ] Modified posts show changes after reload
- [ ] Deleted posts return 404 after reload
- [ ] Draft posts filtered correctly
- [ ] Invalid content doesn't crash server
- [ ] Performance is improved (requests/sec, latency)
- [ ] Memory usage is stable
- [ ] Concurrent requests work during reload
- [ ] No race conditions or crashes
- [ ] Deployment script syncs correctly
- [ ] Special characters handled properly
- [ ] Large number of posts performs well

---

## Next Steps

Once local testing is complete:

1. **Commit your changes:**
   ```bash
   git add .
   git commit -m "Implement post caching with SIGHUP reload"
   git push origin main
   ```

2. **Follow DEPLOYMENT.md** for production deployment

3. **Monitor production** after deployment:
   ```bash
   ssh noosphere journalctl -u rust-blog.service -f
   ```

4. **Test in production** using the same scenarios

---

## Quick Test Script

Save this as `test_cache.sh` for quick testing:

```bash
#!/usr/bin/env bash
set -e

echo "==> Building project"
cargo build

echo "==> Starting server in background"
cargo run &
SERVER_PID=$!
sleep 3

echo "==> Testing endpoints"
curl -f http://localhost:3000/health
curl -f http://localhost:3000/ > /dev/null
echo "✓ Basic endpoints work"

echo "==> Creating test post"
cat > content/posts/2026-01-17-test.md <<EOF
---
title: "Test Post"
slug: "test-post"
date: 2026-01-17
---
Test content
EOF

echo "==> Sending SIGHUP"
kill -HUP $SERVER_PID
sleep 1

echo "==> Verifying new post appears"
curl -f http://localhost:3000/posts/test-post > /dev/null
echo "✓ SIGHUP reload works"

echo "==> Cleaning up"
kill $SERVER_PID
rm content/posts/2026-01-17-test.md

echo "==> All tests passed!"
```

```bash
chmod +x test_cache.sh
./test_cache.sh
```
