#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Configuration from environment
REMOTE_USER="${DEPLOY_USER:-rust-blog}"
REMOTE_HOST="${DEPLOY_HOST:?DEPLOY_HOST environment variable not set}"
REMOTE_CONTENT_PATH="${DEPLOY_CONTENT_PATH:-/var/lib/rust-blog/content}"
LOCAL_CONTENT_PATH="${PROJECT_ROOT}/content"

echo "==> Deploying content to ${REMOTE_HOST}"

# Verify local content exists
if [[ ! -d "$LOCAL_CONTENT_PATH" ]]; then
    echo "ERROR: Local content directory not found: $LOCAL_CONTENT_PATH"
    exit 1
fi

# Test SSH connection
echo "==> Testing SSH connection"
if ! ssh -o BatchMode=yes -o ConnectTimeout=5 \
    "${REMOTE_USER}@${REMOTE_HOST}" true 2>/dev/null; then
    echo "ERROR: Cannot connect to ${REMOTE_HOST}"
    exit 1
fi

# Sync content
echo "==> Syncing content"
rsync -avz --delete \
    --exclude='.DS_Store' \
    --exclude='*.swp' \
    "${LOCAL_CONTENT_PATH}/" \
    "${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_CONTENT_PATH}/"

# Reload cache via SIGHUP
echo "==> Reloading post cache"
ssh "${REMOTE_USER}@${REMOTE_HOST}" \
    "sudo systemctl reload rust-blog.service"

echo "==> Deployment complete!"
