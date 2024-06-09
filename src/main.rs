mod parser;
use std::env;

use parser::parse_args;

fn main() {
    let args: Vec<String> = env::args().collect();
    parse_args(&args);
}