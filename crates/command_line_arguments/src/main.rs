use std::env;

fn main() {
    if env::args().len() < 2 {
        println!("Usage: Program requires at least 1 argument");
        return;
    }

    env::args().enumerate().for_each(|(index, arg)| {
        println!("Argument {} is {}", index, arg);
    });

    println!("arg1 is {}", env::args().nth(1).unwrap());

}
