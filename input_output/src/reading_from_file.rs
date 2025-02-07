use std::fs;

pub fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path).unwrap();
    for line in contents.lines() {
        println!("{}", line);
    }
    contents
}