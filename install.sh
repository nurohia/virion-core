#!/bin/bash
set -e

echo -e "\033[32m[Virion] 安装开始...\033[0m"

# 安装依赖
apt update && apt install -y curl git build-essential pkg-config libssl-dev

# 安装 Rust（如果未安装）
if ! command -v cargo >/dev/null 2>&1; then
  echo -e "\033[33m[+] Rust 未安装，开始安装 Rust...\033[0m"
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source "$HOME/.cargo/env"
  export PATH="$HOME/.cargo/bin:$PATH"
else
  echo -e "\033[32m[✓] Rust 已安装\033[0m"
  source "$HOME/.cargo/env"
  export PATH="$HOME/.cargo/bin:$PATH"
fi

# 清理旧目录
if [ -d "/opt/virion-core" ]; then
  echo -e "\033[33m[!] 检测到旧目录 /opt/virion-core，正在删除...\033[0m"
  rm -rf /opt/virion-core
fi

# 克隆源码
git clone https://github.com/nurohia/virion-core.git /opt/virion-core
cd /opt/virion-core

# 自动清理未使用代码/变量/导入
echo -e "\033[34m[Virion] 正在清理警告...\033[0m"
cargo fix --allow-dirty --allow-staged || true

# 构建 Release
echo -e "\033[34m[Virion] 开始构建 release...\033[0m"
cargo build --release

# 拷贝主程序（注意构建产物是 virion-core）
cp target/release/virion-core /usr/local/bin/virion
chmod +x /usr/local/bin/virion

# 拷贝菜单脚本
cp virion.sh /usr/local/bin/virion.sh
chmod +x /usr/local/bin/virion.sh

# 初始化配置目录
mkdir -p /etc/virion
cp -n config/*.yaml /etc/virion/

echo -e "\033[32m[Virion] 安装完成！\033[0m"
echo -e "\033[32m运行：\033[0m \033[36mvirion.sh\033[0m"
