// Using Rc<T>

use std::rc::Rc;

//use crate::smart_pointers::recursive_type_with_box::List::{Cons, Nil};
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn exercising_rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));


    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercising_rc() {
        exercising_rc();
    }
}