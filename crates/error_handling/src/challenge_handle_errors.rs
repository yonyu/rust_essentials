use rand::prelude::*;
use std::io;

pub fn handle_lower_higher() {
    let mut rng = rand::thread_rng();
    let secret_number =  rng.gen_range(1..101);
    println!("Guess a number between 1 and 100: ");
    loop {
        let mut buffer = String::new();
        let result = match io::stdin().read_line(&mut buffer) {
            Ok(contents) => {},
            Err(error) => {
                println!("please enter a number");
                continue;
            }
        };
        println!("You guessed: {}", buffer);
        let guess: i32 = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number");
                continue;
            }
        };

        if guess < secret_number {
            println!("Too small! Guess bigger:");
        } else if guess > secret_number {
            println!("Too big! Guess smaller");
        } else {
            println!("You win!");
            break;
        }
    }
}