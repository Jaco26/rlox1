/*
  From this chapter http://craftinginterpreters.com/scanning.html
*/

pub struct Scanner {
  source: String
}

impl Scanner {
  pub fn new(source: &str) -> Scanner {
    Scanner { source: String::from(source) }
  }

  pub fn scan_tokens(&self) -> Vec<Token> {
    let tokens = Vec::new();

    tokens
  }
}


#[derive(Debug)]
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


#[derive(Debug)]
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


#[derive(Debug)]
/// Temporary placeholder enum to express all variants of possible `lox` data types
pub enum Literal {
  Str(String),
  Number(f64),
  Identifier(String),
}