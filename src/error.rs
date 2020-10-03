

#[derive(Debug)]
pub struct LoxError {
  pub message: String,
  pub line: usize,
  pub kind: LoxErrorKind,
}

impl LoxError {
  pub fn new(kind: LoxErrorKind, message: &str, line: usize) -> LoxError {
    LoxError { kind, message: String::from(message), line }
  }
}

#[derive(Debug)]
pub enum LoxErrorKind {
  LexicalError
}