use commands::{gen, help, show};

use self::commands::register;

mod commands;

pub fn parse_args(args: &Vec<String>){
    if args.len() == 1{
        println!("Hello! To receive a list of commands you can use -h");
    }
    for el in args.as_slice(){
        if let Ok(result) = match_arg(el, args){
            if !result && el.contains("-"){
                println!("Command not found, try executing -h to view available commands");
                return;
            }
        };
    }
}

fn match_arg(arg: &String, args: &Vec<String>) -> Result<bool, &'static str>{
    match arg.as_str(){
        "-r" => {register(args); Ok(true)}
        "-g" => {gen(args); Ok(true)}
        "-s" => {show(); Ok(true)}
        "-h" => {help(); Ok(true)}
        _ => Ok(false)
    }
}