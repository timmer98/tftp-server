pub struct TftpData {
    opcode: u16,
    block_number: u16,
    data: Vec<u8>,
}

impl TftpData {
    pub fn new(block_number: u16, data: Vec<u8>) -> TftpData{
        return TftpData{opcode: 3, block_number, data};
    }
}