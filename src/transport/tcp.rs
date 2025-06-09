use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::config::Config;
use crate::crypto::chacha::Cipher;
use crate::protocol::packet::{encode_packet, decode_packet, encode_tcp_with_target, decode_tcp_with_target, TYPE_TCP};

pub async fn start_tcp_listener(config: &Config) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(&config.listen).await?;
    log::info!("[TCP] 监听 {}", config.listen);

    loop {
        let (stream, addr) = listener.accept().await?;
        log::info!("[TCP] 接收到来自 {} 的连接", addr);

        let config = config.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_tcp(stream, config).await {
                log::warn!("[TCP] 处理连接错误: {}", e);
            }
        });
    }
}

async fn handle_tcp(mut stream: TcpStream, config: Config) -> tokio::io::Result<()> {
    let cipher = Cipher::new_from_password(&config.password);

    if config.next.is_none() {
        // 服务端模式
        let mut buf = vec![0u8; 2048];
        let n = stream.read(&mut buf).await?;
        let (typ, decrypted) = decode_packet(&buf[..n], &cipher)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "解密失败"))?;
        if typ != TYPE_TCP {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "类型错误"));
        }
        let (target_addr, payload) = decode_tcp_with_target(&decrypted)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "地址解析失败"))?;
        log::info!("[TCP] 转发至：{}", target_addr);
        let mut outbound = TcpStream::connect(&target_addr).await?;
        outbound.write_all(&payload).await?;

        let mut response = vec![0u8; 2048];
        let len = outbound.read(&mut response).await?;
        let encrypted = encode_packet(&response[..len], TYPE_TCP, &cipher).unwrap();
        stream.write_all(&encrypted).await?;
    } else {
        // 客户端模式
        let next = config.next.unwrap();
        let mut remote = TcpStream::connect(&next).await?;
        let target = "example.com:80";
        let encrypted = encode_tcp_with_target(target, b"GET / HTTP/1.0\r\n\r\n", &cipher).unwrap();
        remote.write_all(&encrypted).await?;

        let mut buf = vec![0u8; 2048];
        let n = remote.read(&mut buf).await?;
        let (_typ, decrypted) = decode_packet(&buf[..n], &cipher).unwrap();
        stream.write_all(&decrypted).await?;
    }

    Ok(())
}
