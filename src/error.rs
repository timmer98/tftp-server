pub struct TftpError {
    opcode: u16,
    error_code: u16,
    error_string: String,
}

impl TftpError {
    pub fn new(error_code: u16, error_string: String) -> TftpError {
        return TftpError{opcode: 5, error_code, error_string};
    }
}