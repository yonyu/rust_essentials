//use my_macros::say_hello;
use my_macros::{generate_function, make_answer};

make_answer!();

generate_function! {
    #[inline]
    fn greet() {
        println!("Hello from greet!");
    }
}


use my_macros::hello_macro;

#[hello_macro]
struct Pancakes;

fn main() {
    println!("Hello, world!");

    println!("{}", answer());

    greet();

    Pancakes::hello();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        //say_hello!();
    }

    #[test]
    fn test_pancakes_hello() {
        Pancakes::hello();
    }
}