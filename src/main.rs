mod error;
mod parser;
use std::env;

use parser::parse_args;
use error::{AppErrors, ArgsErrors};

fn main() -> Result<(), AppErrors> {
    let args: Vec<String> = env::args().collect();
    parse_args(&args)
}
