
pub const TYPE_TCP: u8 = 0x01;
pub const TYPE_UDP: u8 = 0x02;

pub fn decode_packet(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

pub fn encode_packet(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}
