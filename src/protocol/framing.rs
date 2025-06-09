// src/protocol/framing.rs

pub fn encode_frame(data: &[u8]) -> Vec<u8> {
    // 简单包长度前缀编码
    let mut framed = Vec::with_capacity(data.len() + 2);
    framed.extend_from_slice(&(data.len() as u16).to_be_bytes());
    framed.extend_from_slice(data);
    framed
}
