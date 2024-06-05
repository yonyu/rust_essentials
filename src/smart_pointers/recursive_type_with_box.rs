// mod recursive_type_with_box 

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::smart_pointers::recursive_type_with_box::List::{Cons, Nil};

pub fn instantiate_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instantiate_list() {
        instantiate_list();
    }
}

