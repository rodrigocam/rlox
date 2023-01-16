use std::fmt;

#[derive(Debug, Clone)]
pub struct LoxError {
    lineno: u32,
    wre: String,
    message: String
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[line {}] Error {} {}", self.lineno, self.wre, self.message)
    }
}
