use std::{fs::{create_dir_all, File, OpenOptions}, io::{Read, Write}};
use dirs::data_local_dir;
use super::match_arg;
use rand::{self, Rng};

pub fn register(args: &[String]){
    let data_path = data_local_dir().unwrap();
    let string_path = data_path.to_str().unwrap();

    create_dir_all(format!("{string_path}/pawwsword")).expect("Error, path couldn't be created.");

    let mut file = File::create(format!("{string_path}/pawwsword/register.txt")).expect("Error, file couldn't be created.");
    
    let register_index = args.iter().position(|x| x == "-r").unwrap();
    if args[register_index] != "-r"{
    }else if args.get(register_index + 1).is_none(){
        println!("A password must be given as an argument to the register command!");
    }else if match_arg(&args[register_index + 1], args){
        return;
    }else {
        let password = &args.get(register_index + 1).unwrap();
        for c in password.chars(){
            let mut b: [u16; 2] = [0; 2];
            c.encode_utf16(&mut b);
            for i in b{
                file.write_all(i.to_string().into_bytes().iter().as_slice()).expect("Bad thing this went oopsies");
                file.write_all(b"\n").expect("Bad thing this went oopsies");
            }
            
        }
    }
}

pub fn gen(args: &[String]){
    let Some(data_path) = data_local_dir() else{
        eprintln!("Failed to get data path!");
        return;
    };
    let Some(string_path) = data_path.to_str() else{
        return;
    };

    if let Err(e) = create_dir_all(format!("{string_path}/pawwsword")){
        eprintln!("Couldn't create path to save file because {e}");
        return;
    };

    if File::open(format!("{string_path}/pawwsword/passwords")).is_err() {
        if let Err(e) = File::create(format!("{string_path}/pawwsword/passwords")){
            eprintln!("Couldn't create file to save passwords because {e}");
            return;
        };
    }

    let Some(generation_index) = args.iter().position(|x| x == "-g") else{
        eprintln!("Couldn't get the position of the password");
        return;
    };
    if args[generation_index] != "-g"{
    }else if args.get(generation_index + 1).is_none(){
        println!("A password must be given as an argument to the generate command!");
    }else if match_arg(&args[generation_index + 1], args) {
        println!("No argument was supplied!");
        return;
    }else {
        let Some(arg) = args.get(generation_index + 1) else{
            eprintln!("No argument was supplied for the password generator.");
            return;
        };
        let mut password = arg.to_owned();
        let code: i32 = rand::thread_rng().gen_range(100_000..999_999);
        for c in code.to_string().chars(){
            let mut pointer = 0;
            let Ok(range): Result<i32, _> = c.to_string().parse() else{
                return;
            };
            for _ in 0..range{
                if pointer > password.len() - 1{
                    pointer = 0;
                }else {
                    pointer += 1;
                }
            }
            password.insert_str(pointer, "UwU");
        }
        let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{string_path}/pawwsword/passwords"))
        .unwrap();

        if let Err(e) = writeln!(file, "{code} {password} ") {
            eprintln!("Couldn't write to file: {e}");
        }

        println!("This is your new secured generated password: {password}");
    }
}

pub fn show(){
    let passwords: &mut String = &mut String::new();

    let Some(data_path) = data_local_dir() else{
        eprintln!("Failed to get data path!");
        return;
    };

    let Some(string_path) = data_path.to_str() else{
        return;
    };

    let Ok(mut file) = File::open(format!("{string_path}/pawwsword/passwords")) else{
        eprintln!("Couldn't read password file (Perhaps no passwords have been saved yet?)");
        return;
    };
    if file.read_to_string(passwords).is_ok(){
        println!("Code:  Password:");
        println!("{passwords}");
    }
}

pub fn help(){
    println!("Available commands:");
    println!("       -g (password): Generate a new secure password from an existent one.");
    println!("       -s: Show your saved passwords.");
}