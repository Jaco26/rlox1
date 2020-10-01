use std::env;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::path::Path;
use core::convert::AsRef;



pub fn main(mut args: env::Args) -> Result<(), io::Error> {
  
  args.next();

  let filename = args.next();

  if let Some(_) = args.next() {

    println!("Ok listen, just give us a path to your code. Thats it! Ok buddy??");

  } else if let Some(filename) = filename {

    run_file(&filename)?;

  } else {

    run_prompt()?;

  }

  Ok(())
}



fn run_file(filepath: impl AsRef<Path>) -> Result<(), io::Error> {

  let source = fs::read_to_string(filepath)?;

  println!("Running file\n\n{}", source);

  Ok(())
}



fn run_prompt() -> Result<(), io::Error> {

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

  }
}

fn run() {

}