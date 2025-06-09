
use crate::protocol::packet::{decode_packet, encode_packet};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run(config: String) -> tokio::io::Result<()> {
    println!("Virion 核心服务已启动！");
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
