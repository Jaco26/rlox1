use std::env;
use std::process;
use rlox1::Rlox1;



fn main() {

    let mut rlox1 = Rlox1::new();

    rlox1.do_it(env::args()).unwrap_or_else(|err| {

        eprintln!("{:?}", err);

        process::exit(1);
    });

}
