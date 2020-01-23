use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use std::fs;

fn main() {
    a_lot_of_matches();
    closures_uber_alles();
    read_username_from_file_matches();
    read_username_from_file_question_operator_long();
    read_username_from_file_question_operator_short();
    read_username_from_file_shortest();
}

fn read_username_from_file_matches() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
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
fn read_username_from_file_question_operator_long() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_question_operator_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn a_lot_of_matches() {
    let f = File::open("a_lot_of_matches.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("a_lot_of_matches.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

fn closures_uber_alles() {
    let f = File::open("closures_uber_alles.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("closures_uber_alles.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
