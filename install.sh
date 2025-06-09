#!/bin/bash
set -e

echo -e "\033[32m[Virion] 安装开始...\033[0m"

apt update && apt install -y curl git build-essential pkg-config libssl-dev

if ! command -v cargo >/dev/null 2>&1; then
  echo "[+] Rust 未安装，开始安装 Rust..."
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source "$HOME/.cargo/env"
fi

git clone https://github.com/nurohia/virion-core.git /opt/virion-core
cd /opt/virion-core
cargo build --release

cp target/release/virion-core /usr/local/bin/virion
chmod +x /usr/local/bin/virion

cp virion.sh /usr/local/bin/virion.sh
chmod +x /usr/local/bin/virion.sh

mkdir -p /etc/virion
cp -n config/*.yaml /etc/virion/

echo -e "\033[32m[Virion] 安装完成！使用：virion.sh 启动\033[0m"
