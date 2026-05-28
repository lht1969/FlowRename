#!/bin/bash
# AdRename 环境安装脚本 - Linux/macOS
# 使用方式: ./scripts/install-deps.sh

set -e

echo "========================================"
echo "  AdRename 环境安装 ($(uname -s))"
echo "========================================"
echo ""

# 检查 Rust 是否已安装
echo "[1/5] 检查 Rust 环境..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo "  [OK] Rust 已安装: $RUST_VERSION"
else
    echo "  [ERROR] Rust 未安装!"
    echo "  请访问 https://rustup.rs 下载并安装 Rust"
    exit 1
fi

# 检查 Node.js 是否已安装
echo "[2/5] 检查 Node.js 环境..."
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    NPM_VERSION=$(npm --version)
    echo "  [OK] Node.js 已安装: v$NODE_VERSION (npm: $NPM_VERSION)"
else
    echo "  [ERROR] Node.js 未安装!"
    echo "  请访问 https://nodejs.org 下载并安装 Node.js (推荐 LTS 版本)"
    exit 1
fi

# 安装系统依赖 (Linux)
if [[ "$(uname -s)" == "Linux" ]]; then
    echo "[3/5] 安装 Linux 系统依赖..."

    if command -v apt-get &> /dev/null; then
        echo "  使用 apt-get 安装依赖..."
        sudo apt-get update
        sudo apt-get install -y \
            libwebkit2gtk-4.1-dev \
            libgtk-3-dev \
            libayatana-appindicator3-dev \
            librsvg2-dev \
            patchelf
        echo "  [OK] 依赖安装完成"
    elif command -v dnf &> /dev/null; then
        echo "  使用 dnf 安装依赖..."
        sudo dnf install -y \
            webkit2gtk4.1-devel \
            gtk3-devel \
            libappindicator-gtk3-devel \
            librsvg2-devel
        echo "  [OK] 依赖安装完成"
    elif command -v pacman &> /dev/null; then
        echo "  使用 pacman 安装依赖..."
        sudo pacman -S --noconfirm \
            webkit2gtk-4.1 \
            gtk3 \
            libappindicator-gtk3 \
            librsvg
        echo "  [OK] 依赖安装完成"
    else
        echo "  [WARNING] 未检测到支持的包管理器，请手动安装以下依赖:"
        echo "    - libwebkit2gtk-4.1-dev"
        echo "    - libgtk-3-dev"
        echo "    - libappindicator-gtk3-dev"
        echo "    - librsvg2-dev"
    fi
fi

# 安装 macOS 特定依赖
if [[ "$(uname -s)" == "Darwin" ]]; then
    echo "[3/5] 检查 Xcode 命令行工具..."
    if command -v xcode-select &> /dev/null; then
        if xcode-select -p &> /dev/null; then
            echo "  [OK] Xcode 命令行工具已安装"
        else
            echo "  [INFO] 正在安装 Xcode 命令行工具..."
            xcode-select --install
            echo "  [OK] 请在弹出的对话框中完成安装"
        fi
    fi
fi

# 安装 Rust 目标平台
echo "[4/5] 检查 Rust 目标平台..."
if [[ "$(uname -s)" == "Linux" ]]; then
    rustup target add x86_64-unknown-linux-gnu
    echo "  [OK] Linux 目标平台已添加"
elif [[ "$(uname -s)" == "Darwin" ]]; then
    rustup target add aarch64-apple-darwin x86_64-apple-darwin
    echo "  [OK] macOS 目标平台已添加"
fi

# 安装项目依赖
echo "[5/5] 安装项目依赖..."
npm install

echo ""
echo "========================================"
echo "  环境配置完成!"
echo "========================================"
echo ""
echo "接下来可以执行以下命令:"
echo "  开发模式: ./scripts/build.sh dev"
echo "  编译应用: ./scripts/build.sh build"
echo "  发布版本: ./scripts/build.sh release"
