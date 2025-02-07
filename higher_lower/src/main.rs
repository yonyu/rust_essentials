use rand::prelude::*;
use std::io; // not included in the prelude

fn main() {
    let mut rng = rand::thread_rng();
    let num =  rng.gen_range(1..101);
    println!("Guess a number between 1 and 100: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    println!("You guessed: {}", buffer);
    let guess : i32 = buffer.trim().parse().unwrap();
    if guess < num {
        println!("Too small!");
    } else if guess > num {
        println!("Too big!");
    }
    else {
        println!("You win!");
    }
    println!("The number was: {}", num);

}
