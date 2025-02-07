use std::fs;
use std::io;

pub fn exercise() {
    let result = fs::read_to_string("input/day1.txt");
    let contents = match result {
        Ok(message) => message,
        Err(e) => //String::from("Nobody knows the ultimate question!")
            {
                match e.kind() {
                    io::ErrorKind::NotFound => String::from("File not found!"),
                    io::ErrorKind::PermissionDenied => String::from("Permission denied!"),
                    _ => panic!("Another type of error: {:?}", e)
                }
            }
    };
    println!("contents is {}", contents);
}