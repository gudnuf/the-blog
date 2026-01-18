#!/usr/bin/env bash
# Quick test to verify caching system works locally

set -e

echo "🧪 Quick Cache Test"
echo "=================="
echo ""

# Check we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Must run from blog root directory"
    exit 1
fi

echo "1. Building project..."
cargo build --quiet || { echo "❌ Build failed"; exit 1; }
echo "   ✓ Build successful"
echo ""

echo "2. Checking for existing posts..."
POST_COUNT=$(find content/posts -name "*.md" 2>/dev/null | wc -l | tr -d ' ')
echo "   Found $POST_COUNT posts in content/posts/"
echo ""

echo "3. Starting server (this will take a few seconds)..."
cargo run > /tmp/blog-test.log 2>&1 &
CARGO_PID=$!

# Wait for server to be ready and find blog-server PID
SERVER_PID=""
for i in {1..10}; do
    if curl -sf http://localhost:3000/health >/dev/null 2>&1; then
        # Find the actual blog-server process PID
        SERVER_PID=$(pgrep -f "target/debug/blog-server" | head -1)
        if [ -z "$SERVER_PID" ]; then
            echo "❌ Could not find blog-server PID"
            kill $CARGO_PID 2>/dev/null || true
            exit 1
        fi
        echo "   ✓ Server started (PID: $SERVER_PID, Cargo PID: $CARGO_PID)"
        break
    fi
    if [ $i -eq 10 ]; then
        echo "❌ Server failed to start within 10 seconds"
        cat /tmp/blog-test.log
        kill $CARGO_PID 2>/dev/null || true
        exit 1
    fi
    sleep 1
done

# Check cache was loaded
echo ""
echo "4. Verifying cache loaded..."
if grep -q "Loaded.*posts into cache" /tmp/blog-test.log; then
    LOADED=$(grep "Loaded.*posts into cache" /tmp/blog-test.log | tail -1)
    echo "   ✓ $LOADED"
else
    echo "   ⚠️  Warning: Cache load message not found in logs"
fi
echo ""

echo "5. Testing endpoints..."
curl -sf http://localhost:3000/health >/dev/null || { echo "   ❌ /health failed"; kill $CARGO_PID 2>/dev/null; exit 1; }
echo "   ✓ GET /health"

curl -sf http://localhost:3000/ >/dev/null || { echo "   ❌ / failed"; kill $CARGO_PID 2>/dev/null; exit 1; }
echo "   ✓ GET /"

curl -sf http://localhost:3000/posts >/dev/null || { echo "   ❌ /posts failed"; kill $CARGO_PID 2>/dev/null; exit 1; }
echo "   ✓ GET /posts"
echo ""

echo "6. Testing SIGHUP reload..."
echo "   Creating test post..."
cat > content/posts/9999-01-01-cache-test.md <<'EOF'
---
title: "Cache Test Post"
slug: "cache-test"
date: 9999-01-01
---
This post tests SIGHUP reload.
EOF

echo "   Sending SIGHUP to PID $SERVER_PID..."
kill -HUP $SERVER_PID
sleep 2

echo "   Checking if new post is accessible..."
if curl -sf http://localhost:3000/posts/cache-test >/dev/null 2>&1; then
    echo "   ✓ New post appeared after SIGHUP"
else
    echo "   ❌ New post not found after reload"
    echo "   Server logs:"
    tail -20 /tmp/blog-test.log
    kill $CARGO_PID 2>/dev/null || true
    rm content/posts/9999-01-01-cache-test.md
    exit 1
fi

# Check logs for reload message
if grep -q "Post cache reloaded successfully" /tmp/blog-test.log; then
    echo "   ✓ Cache reload confirmed in logs"
fi
echo ""

echo "7. Cleaning up..."
rm content/posts/9999-01-01-cache-test.md
kill $CARGO_PID 2>/dev/null || true
wait $CARGO_PID 2>/dev/null || true
rm /tmp/blog-test.log
echo "   ✓ Cleanup complete"
echo ""

echo "=========================================="
echo "✅ All tests passed!"
echo "=========================================="
echo ""
echo "Next steps:"
echo "  • Run './scripts/test_cache.sh' for comprehensive tests"
echo "  • Read TESTING.md for detailed test scenarios"
echo "  • Read DEPLOYMENT.md when ready to deploy"
