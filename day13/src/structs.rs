#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Bit(u8),
    Packets(Vec<Packet>),
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub packet1: Packet, 
    pub packet2: Packet
}
