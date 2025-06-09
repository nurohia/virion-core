
use tokio::net::UdpSocket;

pub async fn start_udp_listener() -> tokio::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:10000").await?;
    let mut buf = [0u8; 1500];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("接收到来自 {} 的 UDP 数据：{:?}", addr, &buf[..len]);
    }
}
