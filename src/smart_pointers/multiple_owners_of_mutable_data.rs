use std::cell::RefCell;
use std::rc::Rc;

// A list that is owned by multiple owners is able to be modified
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn mutate_list_with_multiple_owners() {
    // Create an instance of Rc<RefCell<i32>> and store it in a variable 'value'
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutate_list_with_multiple_owners() {
        mutate_list_with_multiple_owners();
    }
}