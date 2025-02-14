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

// list b and c cannot share the list a
// fn problem() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let _b = Cons(3, Box::new(a));
//     let _c = Cons(4, Box::new(a));
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instantiate_list() {
        instantiate_list();
    }
}

