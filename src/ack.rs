pub struct TftpAck {
    opcode: u16,
    block_number: u16,
}

impl TftpAck {
    pub fn new(block_number: u16) -> TftpAck {
        return TftpAck{opcode: 4, block_number};
    }
}