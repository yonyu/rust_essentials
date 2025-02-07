use std::io;
use std::fs;

pub fn read_and_combine(path1: &str, path2: &str) -> Result<String, io::Error> {
    let mut s1 = fs::read_to_string(path1)?;

    let s2 = match fs::read_to_string(path2) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    s1.push('\n');
    s1.push_str(&s2);

    Ok(s1)
}