use commands::{gen, help, show};

use self::commands::register;

mod commands;

pub fn parse_args(args: &[String]){
    if args.len() == 1{
        println!("Hello! To receive a list of commands you can use -h");
    }
    for el in args{
        if !match_arg(el, args) && el.contains('-'){
                println!("Command not found, try executing -h to view available commands");
                return;
        };
    }
}

fn match_arg(arg: &str, args: &[String]) -> bool{
    match arg{
        "-r" => {register(args); true}
        "-g" => {gen(args); true}
        "-s" => {show(); true}
        "-h" => {help(); true}
        _ => false
    }
}