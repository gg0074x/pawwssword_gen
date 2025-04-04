use commands::{gen, help, show};

use crate::{AppErrors, ArgsErrors};

mod commands;

pub fn parse_args(args: &[String]) -> Result<(), AppErrors> {
    if args.len() == 1 {
        return Err(ArgsErrors::InvalidInput.into());
    }

    let command = &args[1];

    match_arg(&command, args)?;

    Ok(())
}

#[derive(Clone, Copy)]
pub enum Commands {
    Generate,
    Show,
    Help,
}

impl Commands {
    pub fn to_string(self) -> String {
        match self {
            Commands::Generate => "generate",
            Commands::Help => "help",
            Commands::Show => "show",
        }
        .to_owned()
    }

    pub fn to_help(self) -> String {
        match self {
            Commands::Generate => {
                "generate [PASSWORD]: Generate a new safe password from an existent one"
            }
            Commands::Show => "show: Show your generated passwords",
            Commands::Help => "help: Display this message",
        }
        .to_owned()
    }
}

fn match_arg(arg: &str, args: &[String]) -> Result<(), AppErrors> {
    match arg {
        val if val == Commands::Generate.to_string() => gen(args),
        val if val == Commands::Show.to_string() => show(),
        val if val == Commands::Help.to_string() => {
            help();
            Ok(())
        }
        x => Err(ArgsErrors::NotFound(x.to_owned()).into()),
    }
}
