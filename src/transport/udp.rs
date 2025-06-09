use tokio::net::UdpSocket;
use crate::config::Config;
use crate::crypto::chacha::Cipher;
use crate::protocol::packet::{encode_packet, decode_packet, TYPE_UDP};

pub async fn start_udp_listener(config: &Config) -> tokio::io::Result<()> {
    let socket = UdpSocket::bind(&config.listen).await?;
    log::info!("[UDP] 监听 {}", config.listen);

    let mut buf = [0u8; 1500];
    let cipher = Cipher::new_from_password(&config.password);

    loop {
        let (len, peer) = socket.recv_from(&mut buf).await?;
        log::debug!("[UDP] 收到 {} 字节来自 {}", len, peer);

        if config.next.is_none() {
            // 服务端模式，解密获取目标地址 + 数据
            if let Some((typ, decrypted)) = decode_packet(&buf[..len], &cipher) {
                if typ == TYPE_UDP {
                    if let Some((target, payload)) = crate::protocol::packet::decode_tcp_with_target(&decrypted) {
                        let out = UdpSocket::bind("0.0.0.0:0").await?;
                        let _ = out.send_to(&payload, &target).await;
                    }
                }
            }
        } else {
            // 客户端或 relay 模式，封装后发送到下一跳
            let next = config.next.as_ref().unwrap();
            let encrypted = crate::protocol::packet::encode_tcp_with_target(
                "8.8.8.8:53", &buf[..len], &cipher
            ).unwrap();
            let wrapped = encode_packet(&encrypted, TYPE_UDP, &cipher).unwrap();
            socket.send_to(&wrapped, next).await?;
        }
    }
}
