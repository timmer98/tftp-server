pub struct TftpReadRequest {
    opcode: u16,
    filename: String,
    mode: TftpMode,
}

impl TftpReadRequest {
    pub fn new(filename: String, mode: TftpMode) -> TftpRequest {
        return TftpReadRequest{opcode: 1, filename, mode};
    }
}