use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum AppErrors {
    Arguments(#[from] ArgsErrors),
    IO(#[from] std::io::Error),

    #[error("Failed to get data path!")]
    PathNotFound,
    #[error("Couldn't read password file (Perhaps no passwords have been saved yet?)")]
    ReadStoreFile,
    #[error("Couldn't create file to save passwords because {0}")]
    WriteStoreFile(std::io::Error),

    #[error("Cannot parse {0} into i32")]
    CannotParse(char),
    #[error("Couldn't get the position of the password")]
    PasswordPosition,
    #[error("Couldn't parse path into string")]
    PathParse,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ArgsErrors {
    #[error("Two flags cannot be executed at the same time")]
    TwoFlags,
    #[error("The argument {0} is not recognized")]
    NotFound(String),
    #[error("A value was expected for the argument {0}")]
    ValueExpected(&'static str),
    #[error("Hello! To receive a list of commands you can use -h")]
    InvalidInput,
}
