use commands::{gen, help, show};

use self::commands::register;
use crate::{AppError, ArgsError};

mod commands;

pub fn parse_args(args: &[String]) -> Result<(), AppError> {
    if args.len() == 1 {
        return Err(ArgsError::InvalidInput.into());
    }

    for el in args {
        match_arg(el, args)?;
        // if !result && el.contains("-"){
        //     println!("Command not found, try executing -h to view available commands");
        //     return;
        // }
    }

    Ok(())
}

fn match_arg(arg: &str, args: &[String]) -> Result<(), AppError> {
    match arg {
        "-r" => register(args),
        "-g" => gen(args),
        "-s" => show(),
        "-h" => {
            help();
            Ok(())
        }
        x => Err(ArgsError::NotFound(x.to_owned()).into()),
    }
}
