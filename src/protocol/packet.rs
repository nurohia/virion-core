use crate::crypto::chacha::Cipher;

pub const TYPE_TCP: u8 = 0x01;
pub const TYPE_UDP: u8 = 0x02;

pub fn encode_packet(data: &[u8], typ: u8, cipher: &Cipher) -> Option<Vec<u8>> {
    let mut body = vec![typ];
    let encrypted = cipher.encrypt(data)?;
    body.extend_from_slice(&encrypted);
    Some(body)
}

pub fn decode_packet(buf: &[u8], cipher: &Cipher) -> Option<(u8, Vec<u8>)> {
    if buf.len() < 13 {
        return None;
    }
    let typ = buf[0];
    let decrypted = cipher.decrypt(&buf[1..])?;
    Some((typ, decrypted))
}

pub fn encode_tcp_with_target(addr: &str, data: &[u8], cipher: &Cipher) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let addr_bytes = addr.as_bytes();
    if addr_bytes.len() > 255 {
        return None;
    }
    buf.push(addr_bytes.len() as u8);
    buf.extend_from_slice(addr_bytes);
    buf.extend_from_slice(data);
    encode_packet(&buf, TYPE_TCP, cipher)
}

pub fn decode_tcp_with_target(data: &[u8]) -> Option<(String, Vec<u8>)> {
    if data.len() < 1 {
        return None;
    }
    let addr_len = data[0] as usize;
    if data.len() < 1 + addr_len {
        return None;
    }
    let addr = String::from_utf8_lossy(&data[1..1 + addr_len]).to_string();
    let payload = data[1 + addr_len..].to_vec();
    Some((addr, payload))
}
