mod error;
mod parser;
use std::env;

use error::{AppErrors, ArgsErrors};
use parser::parse_args;

fn main() {
    let args: Vec<String> = env::args().collect();
    match parse_args(&args) {
        Ok(()) => (),
        Err(error) => {
            println!("{error}")
        }
    }
}
