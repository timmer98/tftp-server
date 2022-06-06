pub struct TftpWriteRequest {
    opcode: u16,
    filename: String,
    mode: TftpMode,
}

impl TftpWriteRequest {
    pub fn new() -> TftpWriteRequest {
        return TftpWriteRequest {opcode: 2, filename, mode};
    }
}