pub struct Packet {
    packet_type: PacketType,
    payload: Vec<u8>,
}

impl Packet {
    pub fn from_stream(stream: &mut TcpStream) -> Result<Packet> {
        // read packet
    }
}
