#!/bin/bash
CONFIG_DIR="/etc/virion"
BIN="/usr/local/bin/virion"
LOG="/var/log/virion.log"

show_menu() {
  clear
  echo "===== Virion 管理菜单 ====="
  echo "1. 启动客户端"
  echo "2. 启动中继节点"
  echo "3. 启动服务端"
  echo "---------------------------"
  echo "4. 查看配置"
  echo "5. 修改配置"
  echo "6. 重启服务"
  echo "7. 停止所有"
  echo "---------------------------"
  echo "8. 查看日志"
  echo "9. 卸载 Virion"
  echo "0. 退出"
}

start_mode() {
  MODE=$1
  echo "[+] 启动 $MODE..."
  nohup $BIN --config "$CONFIG_DIR/${MODE}.yaml" >> $LOG 2>&1 &
  echo "[✓] $MODE 启动完成"
}

while true; do
  show_menu
  read -rp "请输入选项 [0-9]: " choice
  case "$choice" in
    1) start_mode client ;;
    2) start_mode relay ;;
    3) start_mode server ;;
    4) cat "$CONFIG_DIR"/*.yaml ;;
    5) nano "$CONFIG_DIR/client.yaml" ;;
    6) pkill virion && echo "重启中..." && sleep 1 && $BIN --config "$CONFIG_DIR/client.yaml" & ;;
    7) pkill virion && echo "已停止所有 virion 实例。" ;;
    8) tail -n 100 "$LOG" ;;
    9) rm -rf /opt/virion-core /usr/local/bin/virion /usr/local/bin/virion.sh /etc/virion && echo "已卸载。" ;;
    0) exit ;;
    *) echo "无效选项，请重试。" ;;
  esac
  read -rp "按任意键继续..." _key
done
