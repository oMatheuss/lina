use std::fmt::Display;

use crate::lexer::LexerPosition;

pub struct Error {
    message: String,
    row: usize,
    col: usize,
    file_name: String,
}

impl Error {
    pub fn new(message: &str, pos: &LexerPosition) -> Self {
        Error {
            message: String::from(message),
            row: pos.line_num,
            col: pos.curr_char - pos.line_start,
            file_name: pos.file_name.clone(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Erro: {}", self.message)?;
        writeln!(f, "--> {} {}:{}", self.file_name, self.row, self.col)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
