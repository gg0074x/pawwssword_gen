use commands::{gen, help, show};

use self::commands::register;
use crate::{AppErrors, ArgsErrors};

mod commands;

pub fn parse_args(args: &[String]) -> Result<(), AppErrors> {
    if args.len() == 1 {
        return Err(ArgsErrors::InvalidInput.into());
    }

    for el in args {
        if el.contains('-'){
            match_arg(el, args)?;
        }
        // if !result && el.contains("-"){
        //     println!("Command not found, try executing -h to view available commands");
        //     return;
        // }
    }

    Ok(())
}

fn match_arg(arg: &str, args: &[String]) -> Result<(), AppErrors> {
    match arg {
        "-r" => register(args),
        "-g" => gen(args),
        "-s" => show(),
        "-h" => {
            help();
            Ok(())
        }
        x => Err(ArgsErrors::NotFound(x.to_owned()).into()),
    }
}
