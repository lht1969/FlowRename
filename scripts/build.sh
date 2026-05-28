#!/bin/bash
# AdRename 构建脚本 - Linux/macOS
# 使用方式: ./scripts/build.sh [dev|build|release]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "========================================"
echo "  AdRename 构建脚本 ($(uname -s))"
echo "========================================"
echo ""

MODE="${1:-build}"

case "$MODE" in
    dev)
        echo "[INFO] 启动开发模式..."
        npm run tauri dev
        ;;
    build)
        echo "[INFO] 执行开发构建..."
        npm run tauri build
        ;;
    release)
        echo "[INFO] 执行发布构建..."
        npm run tauri build
        ;;
    *)
        echo "[ERROR] 无效参数: $MODE"
        echo "用法: $0 [dev|build|release]"
        exit 1
        ;;
esac

if [ $? -ne 0 ]; then
    echo "[ERROR] 构建失败!"
    exit 1
fi

echo ""
echo "[SUCCESS] 构建完成!"
echo "构建产物位于: $PROJECT_ROOT/src-tauri/target/release/bundle"
