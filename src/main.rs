mod error;
mod parser;
use std::env;

use parser::parse_args;
use error::*;

fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();
    parse_args(&args)
}
