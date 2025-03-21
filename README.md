# IPLens

IPLens 是一个跨平台的命令行工具，用于检测和显示 IP 相关信息。

## 安装方法

### 使用安装脚本 (推荐)

Linux 和 macOS 用户可以使用以下命令一键安装：

```bash
curl -fsSL https://raw.githubusercontent.com/0x1cc4/iplens/main/install.sh | bash
```

### 手动安装

1. 从 [GitHub Releases](https://github.com/0x1cc4/iplens/releases) 下载最新版本：

macOS 用户：
```bash
# Intel Mac
curl -LO https://github.com/0x1cc4/iplens/releases/latest/download/iplens-macos-x86_64
chmod +x iplens-macos-x86_64
sudo mv iplens-macos-x86_64 /usr/local/bin/iplens

# Apple Silicon (M1/M2) Mac
curl -LO https://github.com/0x1cc4/iplens/releases/latest/download/iplens-macos-arm64
chmod +x iplens-macos-arm64
sudo mv iplens-macos-arm64 /usr/local/bin/iplens
```

Linux 用户：
```bash
curl -LO https://github.com/0x1cc4/iplens/releases/latest/download/iplens-linux-x86_64
chmod +x iplens-linux-x86_64
sudo mv iplens-linux-x86_64 /usr/local/bin/iplens
```

2. 验证安装：
```bash
iplens --version
```

### 从源码安装

需要先安装 Rust 工具链：

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆仓库
git clone https://github.com/0x1cc4/iplens.git
cd iplens

# 编译安装
cargo install --path .
```

## 功能特点

- 检测并显示本地 IP 地址（优先显示 IPv4，过滤回环地址）
<!-- - 获取公网 IP 地址（支持多个 API 端点自动切换） -->
<!-- - 检测 VPN 连接状态和 IP 地址 -->
- 显示系统 DNS 服务器配置
- 支持表格和 JSON 两种输出格式
<!-- - 跨平台支持（Windows/macOS/Linux） -->

## 使用方法

默认表格输出

```bash
iplens
```

JSON 输出

```bash
iplens --json
```

