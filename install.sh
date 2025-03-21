#!/bin/bash

set -e

# 检测系统和架构
OS="$(uname -s)"
ARCH="$(uname -m)"

# 确定下载链接
GITHUB_REPO="0x1cc4/iplens"
LATEST_RELEASE_URL="https://github.com/$GITHUB_REPO/releases/latest/download"

if [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ]; then
        BINARY_URL="$LATEST_RELEASE_URL/iplens-macos-arm64"
    else
        BINARY_URL="$LATEST_RELEASE_URL/iplens-macos-x86_64"
    fi
elif [ "$OS" = "Linux" ]; then
    BINARY_URL="$LATEST_RELEASE_URL/iplens-linux-x86_64"
else
    echo "不支持的操作系统: $OS"
    exit 1
fi

# 创建临时目录
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

echo "下载 iplens..."
if command -v curl > /dev/null 2>&1; then
    curl -fsSL "$BINARY_URL" -o iplens
else
    wget -q "$BINARY_URL" -O iplens
fi

# 设置执行权限
chmod +x iplens

# 移动到 bin 目录
if [ -w "/usr/local/bin" ]; then
    mv iplens /usr/local/bin/
else
    sudo mv iplens /usr/local/bin/
fi

# 清理临时目录
cd - > /dev/null
rm -rf "$TMP_DIR"

echo "iplens 已成功安装到 /usr/local/bin/iplens"
echo "运行 'iplens --help' 查看使用说明" 