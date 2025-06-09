#!/bin/bash
set -e

echo -e "\033[32m[Virion] 安装开始...\033[0m"

# 安装依赖
apt update && apt install -y curl git build-essential pkg-config libssl-dev

# 安装 Rust（如果未安装）
if ! command -v cargo >/dev/null 2>&1; then
  echo "[+] Rust 未安装，开始安装 Rust..."
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source "$HOME/.cargo/env"
else
  echo "[✓] Rust 已安装"
  source "$HOME/.cargo/env"
fi

# 清理旧目录（避免 git 报错）
if [ -d "/opt/virion-core" ]; then
  echo "[!] 检测到旧目录 /opt/virion-core，正在删除..."
  rm -rf /opt/virion-core
fi

# 克隆源码并构建
git clone https://github.com/nurohia/virion-core.git /opt/virion-core
cd /opt/virion-core
cargo build --release

# 安装核心程序和菜单脚本
cp target/release/virion /usr/local/bin/virion
chmod +x /usr/local/bin/virion

cp virion.sh /usr/local/bin/virion.sh
chmod +x /usr/local/bin/virion.sh

# 配置目录初始化
mkdir -p /etc/virion
cp -n config/*.yaml /etc/virion/

echo -e "\033[32m[Virion] 安装完成！使用：virion.sh 启动\033[0m"
