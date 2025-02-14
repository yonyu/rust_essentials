use std::fs;
use std::io::prelude::*; //use std::io::Write; // import the Write trait from the io module

pub fn write_speech() {
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard. \n");

    // https://doc.rust-lang.org/std/fs/struct.File.html
    fs::write("speech.txt", speech).expect("Unable to write file");

    let mut planets = String::new();
    planets.push_str("Mercury\n");
    planets.push_str("Venus\n");
    planets.push_str("Earth\n");
    planets.push_str("Mars\n");
    planets.push_str("Jupiter\n");
    planets.push_str("Saturn\n");
    planets.push_str("Uranus\n");
    planets.push_str("Neptune\n");
    let mut file = fs::File::create("planets.txt").expect("Unable to create file");
    file.write_all(planets.as_bytes()).expect("Unable to write file");

    let mut file = fs::OpenOptions::new().append(true).create(true).open("planets.txt").unwrap();
    file.write(b"\nPlutoo").expect("unable to write to file");
}