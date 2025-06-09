# Virion Core

**Virion** 是一款使用 Rust 编写的轻量级、高性能、支持 TCP/UDP 的多跳加密隧道转发工具，采用自研 ChaCha20-Poly1305 加密，无需 TLS，支持游戏代理、隐蔽通信、UDP 映射等高性能场景。

---

## ✨ 功能特性

- ✅ 支持 **TCP / UDP 转发**
- ✅ 自研 **ChaCha20-Poly1305 加密协议**（无需 TLS）
- ✅ 支持 **动态目标地址封装**
- ✅ 支持 **多跳 relay 中转**
- ✅ 提供 **YAML 配置 / CLI 菜单 / 一键安装**
- ✅ 支持 **systemd 后台运行**

---

## 📦 一键安装

推荐使用以下命令一键安装并启动管理菜单：

```bash
bash <(curl -fsSL https://raw.githubusercontent.com/nurohia/virion-core/main/install.sh) && virion.sh
