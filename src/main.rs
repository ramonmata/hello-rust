#![allow(unused)]

use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::error::{Error};
use std::net::IpAddr;


// Box<dyn Error>  is a trait object -- in this case fits any kind of error
fn main() -> Result<(), Box<dyn Error>>{
    let file_path = "Hello.txt";

    // let i:f32 = "12710.1".parse()?;
    // let home: IpAddr = "1271.0.1".parse()?;
    
    // Using Match
    let f = File::open(file_path);
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(file) => file,
                Err(error) => panic!("Problem creating file: {:?}", error)
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error)
            }
        }
    };

    // println!("File Opened: {:?}", f.metadata());

//     // Using Closures
//     let f = File::open(file_path).unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create(file_path).unwrap_or_else(|error| {
//                 panic!("Problem creating file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening file: {:?}", error);
//         }
//     });

//     println!("File Opened: {:?}", f.metadata());

    let f = File::open(file_path).unwrap(); // If result is Ok, we get value inside Ok
    let f = File::open(file_path).expect("File does not exist"); // IF err We specify the error message in panic, else the value inside Ok

    let user_name = read_username_from_file_super_shortcut_plus(file_path)?;
    println!("User Name: {}", user_name);

    Ok(())
}


// Propagating errors

fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let f = File::open(file_name);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Shortcut for Propagating errors
fn read_username_from_file_shortcut(file_name: &str) -> Result<String, io::Error> {
    let mut f = File::open(file_name)?;
    let mut user_name = String::new();
    f.read_to_string(&mut user_name)?;
    Ok(user_name)
}


// Even shorter
fn read_username_from_file_super_shortcut(file_name: &str) -> Result<String, io::Error> {
    let mut user_name = String::new();
    File::open(file_name)?.read_to_string(&mut user_name)?;
    Ok(user_name)
}

// Even Even shorter
fn read_username_from_file_super_shortcut_plus(file_name: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(file_name)
}