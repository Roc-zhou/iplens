# IPLens

IPLens 是一个跨平台的命令行工具，用于检测和显示 IP 相关信息。

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

