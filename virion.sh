#!/bin/bash

CONFIG_DIR="/etc/virion"
RULE_FILE="$CONFIG_DIR/rules.yaml"
LOG_FILE="/var/log/virion.log"

mkdir -p "$CONFIG_DIR"
[ ! -f "$RULE_FILE" ] && echo -e "inbounds:\n  []\n\noutbounds:\n  []" > "$RULE_FILE"

GREEN="\033[32m"
RESET="\033[0m"

add_direct_forward() {
    read -p "请输入入口监听端口: " listen_port
    read -p "请输入目标地址 IP:端口: " target

    cat >> "$RULE_FILE" <<EOF

- type: tcp
  listen: 0.0.0.0:$listen_port
EOF

    cat >> "$RULE_FILE" <<EOF

- type: tcp
  remote: $target
EOF

    echo -e "${GREEN}规则添加成功：$listen_port -> $target${RESET}"
}

add_relay_forward() {
    read -p "请输入入口监听端口 (如 A 机): " entry_port
    read -p "请输入中转出口地址 (B 机 IP:端口): " middle_target
    read -p "请输入最终目标地址 (C 机 IP:端口): " final_target

    cat >> "$RULE_FILE" <<EOF

- type: tcp
  listen: 0.0.0.0:$entry_port
EOF

    cat >> "$RULE_FILE" <<EOF

- type: tcp
  remote: $middle_target
EOF

    echo -e "${GREEN}入口中转规则添加成功：$entry_port -> $middle_target${RESET}"
    echo -e "${GREEN}请在 B 机添加出口规则将其转发到：$final_target${RESET}"
}

uninstall_virion() {
    echo -e "${GREEN}卸载中...${RESET}"
    systemctl stop virion 2>/dev/null
    rm -rf /usr/local/bin/virion /usr/local/bin/virion.sh /etc/virion /opt/virion-core "$LOG_FILE"
    echo -e "${GREEN}Virion 已彻底卸载。${RESET}"
}

main_menu() {
    clear
    echo -e "${GREEN}==== Virion 一键管理脚本 ====${RESET}"
    echo "1) 添加入口直出规则"
    echo "2) 添加中转规则"
    echo "3) 查看当前规则"
    echo "4) 删除规则 (请手动编辑 YAML)"
    echo "5) 启动服务"
    echo "6) 查看日志"
    echo "7) 卸载服务"
    echo "8) 退出"
    echo "=============================="
    read -p "请输入选项 [1-8]: " choice

    case $choice in
        1) add_direct_forward;;
        2) add_relay_forward;;
        3) cat "$RULE_FILE";;
        4) echo "请手动编辑 $RULE_FILE 删除条目...";;
        5) nohup virion-core -c "$RULE_FILE" >> "$LOG_FILE" 2>&1 & echo "[✓] 已启动 Virion";;
        6) tail -n 100 "$LOG_FILE";;
        7) uninstall_virion;;
        8) exit 0;;
        *) echo "无效选项";;
    esac

    echo ""
    read -p "按回车继续..."
    main_menu
}

main_menu
