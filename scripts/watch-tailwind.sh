#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "Watching Tailwind CSS..."
echo "Input:  static/css/input.css"
echo "Output: static/css/tailwind.css"
echo ""

tailwindcss -i ./static/css/input.css -o ./static/css/tailwind.css --watch
