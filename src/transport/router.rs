use tokio::net::TcpStream;
use crate::config::Config;
use crate::crypto::chacha::Cipher;
use crate::protocol::packet::{decode_packet, encode_packet, TYPE_TCP, TYPE_UDP};

pub async fn relay_tcp(mut inbound: TcpStream, config: &Config) -> tokio::io::Result<()> {
    let cipher = Cipher::new_from_password(&config.password);
    let next = config.next.as_ref().unwrap();
    let mut next_stream = TcpStream::connect(next).await?;

    // 接收数据 → 解密 → 再加密 → 发往下一跳
    let mut buf = vec![0u8; 2048];
    let n = inbound.readable().await?;
    let n = inbound.try_read(&mut buf).unwrap_or(0);
    if n == 0 {
        return Ok(());
    }

    if let Some((typ, decrypted)) = decode_packet(&buf[..n], &cipher) {
        let re_encrypted = encode_packet(&decrypted, typ, &cipher).unwrap();
        next_stream.writable().await?;
        next_stream.try_write(&re_encrypted)?;
    }

    Ok(())
}
