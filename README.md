# Virion Core

Virion 是一个使用 Rust 编写的轻量级、高性能、支持多跳加密的万能 TCP/UDP 隧道转发工具，支持多节点 relay、UDP 映射、无 TLS、自定义加密协议，适合游戏代理、隐蔽通信等高性能场景。

## ✨ 功能特性

- ✅ TCP / UDP 转发
- ✅ 自研 ChaCha20-Poly1305 加密（无 TLS）
- ✅ 支持动态目标地址封装
- ✅ 多跳 relay 中转
- ✅ YAML 配置 / CLI 菜单 / 一键安装
- ✅ systemd 支持后台运行

## 📦 安装方式

```bash
bash install.sh
```

安装后可使用：

```bash
virion.sh   # 启动管理菜单
```

## 🧾 配置模板示例（config/client.yaml）

```yaml
mode: client
listen: "127.0.0.1:1080"
password: "supersecret"
next: "1.2.3.4:40000"
```

## 🛠️ 编译方式

```bash
cargo build --release
```

## 🔁 多跳架构示例（A ~ B ~ C）

```text
[Client] → [Relay B] → [Server C] → [目标地址]
```

每一跳运行 `virion --config relay.yaml`，配置不同。

## 📂 systemd 使用

```bash
cp systemd/virion-client.service /etc/systemd/system/
systemctl daemon-reexec
systemctl enable --now virion-client
```

## 📃 LICENSE

MIT
