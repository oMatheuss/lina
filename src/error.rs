use std::fmt::Display;

pub struct Error {
    message: String,
    row: usize,
    col: usize,
    file_name: String,
}

impl Error {
    pub fn new(message: &str, file: &str, row: usize, col: usize) -> Self {
        Self {
            message: String::from(message),
            row,
            col,
            file_name: String::from(file),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Erro: {}", self.message)?;
        writeln!(f, "--> {} {}:{}", self.file_name, self.row, self.col)
    }
}
