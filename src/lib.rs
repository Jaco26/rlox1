mod scanner;
mod error;
mod util;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::process;
use std::path::Path;
use core::convert::AsRef;

pub struct Rlox1 {
  has_error: bool,
}

impl Rlox1 {

  pub fn new() -> Rlox1 {
    Rlox1 { has_error: false }
  }

  pub fn do_it(&mut self, mut args: env::Args) -> Result<(), io::Error> {
    args.next();
    let filename = args.next();
    if let Some(_) = args.next() {
      println!("Ok listen, just give us a path to your code. Thats it! Ok buddy??");
    } else if let Some(filename) = filename {
      self.run_file(&filename)?;
    } else {
      self.run_prompt()?;
    }
    Ok(())
  }
  

  fn run_file(&mut self, filepath: impl AsRef<Path>) -> Result<(), io::Error> {
    let source = fs::read_to_string(filepath)?;
    self.run(&source);
    Ok(())
  }
  
  
  fn run_prompt(&mut self) -> Result<(), io::Error> {
    loop {
      let user_input = {
        print!("> ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        buffer.pop();
        buffer
      };
      println!("{}", user_input);
      self.run(&user_input);
      self.has_error = false;
    }
  }
  
  
  fn run(&mut self, source: &str) {
    use scanner::Scanner;

    let mut scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens().unwrap_or_else(|err| {
      self.error(err.line, err.kind, &err.message);
      vec![]
    });
  
    for token in tokens {
      println!("{:?}", token);
    }

    if self.has_error {
      process::exit(65);
    }
  }
  
  
  fn error(&mut self, line: usize, kind: error::LoxErrorKind, message: &str) {
    self.report(line, kind, "", message);
  }
  
  fn report(&mut self, line: usize, kind: error::LoxErrorKind, error_location: &str, message: &str) {
    eprintln!("[line {}] {:?} {} : {}", line, kind, error_location, message);
    self.has_error = true
  }
}

