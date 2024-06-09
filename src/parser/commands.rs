use crate::{AppError, ArgsError};
use dirs::data_local_dir;
use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{Read, Write},
};

use super::match_arg;
use rand::{self, Rng};

pub fn register(args: &[String]) -> Result<(), AppError> {
    let data_path = data_local_dir().ok_or(AppError::PathNotFound)?;

    create_dir_all(format!("{data_path:?}/pawwsword"))?;

    let mut file = File::create(format!("{data_path:?}/pawwsword/passwords"))
        .map_err(AppError::WriteStoreFile)?;

    let register_index = args.iter().position(|x| x == "-r").unwrap();
    if args.get(register_index + 1).is_none() {
        return Err(ArgsError::ValueExpected("-r").into());
    } else if match_arg(&args[register_index + 1], args).is_ok() {
        return Err(ArgsError::TwoFlags.into());
    } else {
        let password = args
            .get(register_index + 1)
            .ok_or(ArgsError::ValueExpected("-r"))?;
        for c in password.chars() {
            let mut b: [u16; 2] = [0; 2];
            c.encode_utf16(&mut b);
            for i in b {
                file.write_all(i.to_string().into_bytes().iter().as_slice())?;
                file.write_all(b"\n")?;
            }
        }
    }

    Ok(())
}

pub fn gen(args: &[String]) -> Result<(), AppError> {
    let data_path = data_local_dir().ok_or(AppError::PathNotFound)?;

    create_dir_all(format!("{data_path:?}/pawwsword"))?;

    if File::open(format!("{data_path:?}/pawwsword/passwords")).is_err() {
        File::create(format!("{data_path:?}/pawwsword/passwords"))
            .map_err(AppError::WriteStoreFile)?;
    }

    let generation_index = args
        .iter()
        .position(|x| x == "-g")
        .ok_or(AppError::PasswordPosition)?;

    if args.get(generation_index + 1).is_none() {
        return Err(ArgsError::ValueExpected("-g").into());
    } else if match_arg(&args[generation_index + 1], args).is_ok() {
        return Err(ArgsError::TwoFlags.into());
    } else {
        let mut password = args
            .get(generation_index + 1)
            .unwrap_or(&String::from("Password"))
            .to_owned();
        let code: i32 = rand::thread_rng().gen_range(100000..999999);
        for c in code.to_string().chars() {
            let mut pointer = 0;
            let range = c
                .to_string()
                .parse::<i32>()
                .map_err(|_| AppError::CannotParse(c))?;
            for _ in 0..range {
                if pointer > password.len() - 1 {
                    pointer = 0;
                } else {
                    pointer += 1;
                }
            }
            password.insert_str(pointer, "UwU");
        }
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{data_path:?}/pawwsword/passwords"))?;

        writeln!(file, "{code} {password} ")?;

        println!("This is your new secured generated password: {password}");
    }
    Ok(())
}

pub fn show() -> Result<(), AppError> {
    let mut passwords = String::default();

    let data_path = data_local_dir().ok_or(AppError::PathNotFound)?;

    let mut file = File::open(format!("{data_path:?}/pawwsword/passwords"))
        .map_err(|_| AppError::ReadStoreFile)?;
    file.read_to_string(&mut passwords)?;

    println!("Code:  Password:");
    println!("{passwords}");

    Ok(())
}

pub fn help() {
    println!("Available commands:");
    println!("       -g (password): Generate a new secure password from an existent one.");
    println!("       -s: Show your saved passwords.");
}
