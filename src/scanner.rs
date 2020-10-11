/*
  From this chapter http://craftinginterpreters.com/scanning.html
*/

use crate::error::LoxError;
use crate::error::LoxErrorKind;
use crate::util::ternary;

pub struct Scanner {
  source: String,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: usize,
}

impl Scanner {
  pub fn new(source: &str) -> Scanner {
    Scanner {
      source: String::from(source),
      tokens: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
    }
  }

  pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
    while !self.is_at_end() {
      // We are at the beginning of the next lexeme
      self.start = self.current;
      self.scan_token()?;
    }
    self.tokens.push(Token::new(TokenKind::EOF, "", None, self.line));
    Ok(self.tokens.clone())
  }


  fn scan_token(&mut self) -> Result<(), LoxError> {
    use TokenKind::*;

    match self.advance_current() {
      // Single-character lexemes
      '(' => self.add_token(LeftParen, None),
      ')' => self.add_token(RightParen, None),
      '{' => self.add_token(LeftBrace, None),
      '}' => self.add_token(RightBrace, None),
      ',' => self.add_token(Comma, None),
      '.' => self.add_token(Dot, None),
      '-' => self.add_token(Minus, None),
      '+' => self.add_token(Plus, None),
      ';' => self.add_token(Semicolon, None),
      '*' => self.add_token(Star, None),
      // Multi-character lexemes
      '!' => {
        let kind = match self.match_current('=') {
          true => BangEqual,
          false => Bang,
        };
        self.add_token(kind, None);
      }
      '=' => {
        let kind = match self.match_current('=') {
          true => EqualEqual,
          false => Equal,
        };
        self.add_token(kind, None);
      }
      '<' => {
        let kind = match self.match_current('=') {
          true => LessEqual,
          false => Less,
        };
        self.add_token(kind, None);
      }
      '>' => {
        let kind = match self.match_current('=') {
          true => GreaterEqual,
          false => Greater,
        };
        self.add_token(kind, None);
      }
      '/' => {
        if self.match_current('/') {
          while self.lookahead() != '\n' && !self.is_at_end() {
            self.advance_current();
          }
        } else {
          self.add_token(Slash, None);
        }
      }
      ' ' | '\r' | '\t' => {}
      '\n' => {
        self.line += 1;
      }
      '"' => {
        self.string()?;
      }
      _ => {
        return Err(LoxError::new(LoxErrorKind::LexicalError, "Unexpected character.", self.line));
      }
    };
    Ok(())
  }

  fn string(&mut self) -> Result<(), LoxError> {
    while self.lookahead() != '"' && !self.is_at_end() {
      if self.lookahead() == '\n' {
        self.line += 1;
      }
      self.advance_current();
    }

    if self.is_at_end() {
      return Err(LoxError::new(LoxErrorKind::LexicalError, "Unterminated string.", self.line));
    }

    self.advance_current();

    let value = &self.source[(self.start + 1)..(self.current - 1)];
    let value = Literal::Str(String::from(value));
    self.add_token(TokenKind::Str, Some(value));

    Ok(())
  }

  fn add_token(&mut self, kind: TokenKind, literal: Option<Literal>) {
    let text = &self.source[self.start..self.current];
    self.tokens.push(Token::new(kind, text, literal, self.line));
  }

  fn advance_current(&mut self) -> char {
    self.current += 1;
    self.source.chars().nth(self.current - 1).unwrap()
  }

  fn match_current(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      return false
    }
    if self.source.chars().nth(self.current).unwrap() != expected {
      return false;
    }
    self.current += 1;
    true
  }

  fn lookahead(&self) -> char {
    if self.is_at_end() {
      return '\0'
    }
    self.source.chars().nth(self.current).unwrap()
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }
}


#[derive(Debug, Clone)]
/// The first step in any compiler or interpreter is scanning. The scanner takes in raw source code
/// as a series of characters and groups it into a series of chunks we call tokens. These are meaningful
/// "words" and "punctuation" that make up a language's grammar.
///
/// Lexical Analyssis:
/// Our job is to scan through a list of characters and group them together into
/// the smallest sequences that still represent something. Each of the blobs of 
/// characters is called a "lexeme".
/// 
/// if given `var language = "lox";`
/// our lexems would be [var, language, =, "lox", ;]
/// 
/// Lexemes are only the raw substrings of the source code. However, in the process of grouping character
/// sequences into lexems, we also stumble upon some other useful information. When we take the lexeme and
/// bundle it together with that other data, the result is a "token".
pub struct Token {
  /// The raw substring of the source code
  lexeme: String,
  /// Keywords are part of the shape of the language's grammar, so the parser
  /// often has code like, "if the next token is `while` then do..." That means 
  /// the parser wants to know not just that it has a lexeme for some identifier but
  /// that it has a _reserved_ word and _which_ keyword it is.
  /// 
  /// At the point that we recognize a lexeme, we also remember which _kind_ of lexeme
  /// it represents. We have a different `TokenKind` variant of each keyword, operator,
  /// bit of punctuation, and literal type.
  kind: TokenKind,
  /// There are lexemes for literal values – numbers and strings and the like. Since
  /// the scanner has to walk each character in the literal to correctly identify it,
  /// it can also convert it to the real runtime value that will be used by the interpreter
  /// later
  literal: Option<Literal>,
  /// In our simple interpreter we'll only note which line the token appears on, but more
  /// sophisticated implementations include the column and length too. This is useful for
  /// error tracking.
  line: usize,
}

impl Token {
  pub fn new( kind: TokenKind, lexeme: &str, literal: Option<Literal>, line: usize) -> Token {
    Token { lexeme: String::from(lexeme), kind, literal, line }
  }

  pub fn to_string(&self) -> String {
    format!("{:?} {} {:?}", self.kind, self.lexeme, self.literal)
  }
}

#[derive(Debug, Clone)]
pub enum TokenKind {
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, Str, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  EOF
}


#[derive(Debug, Clone)]
/// Temporary placeholder enum to express all variants of possible `lox` data types
pub enum Literal {
  Str(String),
  Number(f64),
  Identifier(String),
}