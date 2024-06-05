// implementing customized smart pointer

use std::ops::Deref;

struct MyBox<T> (T); // tuple struct

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let my = MyBox::new(5);
        println!("{}", *my);
    }

    #[test]
    fn test_hello() {
        // deref coercion
        let my = MyBox::new(String::from("Rust"));
        hello(&my);
    }
}