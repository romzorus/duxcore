#[derive(Debug)]
pub struct CmdResult {
    pub exitcode: i32,
    pub stdout: String,
}

impl CmdResult {
    pub fn new() -> CmdResult {
        CmdResult {
            exitcode: 0,
            stdout: String::new(),
        }
    }
}
