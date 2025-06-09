
pub fn encode_frame(data: &[u8]) -> Vec<u8> {
    let mut frame = Vec::new();
    frame.extend_from_slice(&(data.len() as u32).to_be_bytes());
    frame.extend_from_slice(data);
    frame
}
