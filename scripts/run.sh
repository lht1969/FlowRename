#!/bin/bash
# AdRename 执行脚本 - Linux/macOS
# 使用方式: ./scripts/run.sh [--debug]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "========================================"
echo "  AdRename 执行脚本 ($(uname -s))"
echo "========================================"
echo ""

DEBUG_FLAG=""
if [ "$1" = "--debug" ]; then
    DEBUG_FLAG="--debug"
    echo "[INFO] 启动调试模式..."
else
    echo "[INFO] 启动应用..."
fi

npm run tauri dev $DEBUG_FLAG
