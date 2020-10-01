use std::env;
use std::process;
use rlox1;



fn main() {

    rlox1::main(env::args()).unwrap_or_else(|err| {

        eprintln!("{:?}", err);

        process::exit(1);
    });

}
