use std::fs;

pub fn exercise() {
    //let contents = fs::read_to_string("input/day1.txt").unwarp();
    let contents = fs::read_to_string("input/day1.txt").expect("Should have been able to read the file");
    println!("contents is {}", contents);
}