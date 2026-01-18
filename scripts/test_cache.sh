#!/usr/bin/env bash
set -e

echo "==> Building project"
cargo build

echo "==> Starting server in background"
cargo run &
SERVER_PID=$!
echo "Server PID: $SERVER_PID"
sleep 3

echo "==> Testing endpoints"
curl -f http://localhost:3000/health || { echo "Health check failed"; kill $SERVER_PID; exit 1; }
curl -f http://localhost:3000/ > /dev/null || { echo "Index failed"; kill $SERVER_PID; exit 1; }
echo "✓ Basic endpoints work"

echo "==> Creating test post"
cat > content/posts/2026-01-17-test.md <<EOF
---
title: "Test Post"
slug: "test-post"
date: 2026-01-17
---
Test content for SIGHUP reload verification.
EOF

echo "==> Sending SIGHUP"
kill -HUP $SERVER_PID
sleep 2

echo "==> Verifying new post appears"
curl -f http://localhost:3000/posts/test-post > /dev/null || { echo "New post not found"; kill $SERVER_PID; exit 1; }
echo "✓ SIGHUP reload works"

echo "==> Testing post modification"
cat > content/posts/2026-01-17-test.md <<EOF
---
title: "Test Post - MODIFIED"
slug: "test-post"
date: 2026-01-17
---
Modified content.
EOF

kill -HUP $SERVER_PID
sleep 2

curl -s http://localhost:3000/posts/test-post | grep -q "MODIFIED" || { echo "Modified content not found"; kill $SERVER_PID; exit 1; }
echo "✓ Post modification detected"

echo "==> Testing post deletion"
rm content/posts/2026-01-17-test.md
kill -HUP $SERVER_PID
sleep 2

if curl -f http://localhost:3000/posts/test-post > /dev/null 2>&1; then
  echo "Deleted post still accessible"
  kill $SERVER_PID
  exit 1
fi
echo "✓ Post deletion detected"

echo "==> Cleaning up"
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null || true

echo ""
echo "================================================"
echo "✓ All cache tests passed!"
echo "================================================"
