use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    // A Node owns its children, and we want to share that ownership with variables
    // so we can access each Node in the tree directly, so we define Vec<Rc<Node>>.
    // We also want to modify which nodes are children of other nodes, so we use RefCell
    // around Vec<Rc<Node>>, RefCell<Vec<Rc<Node>>>.
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn demo_create_tree() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_create_tree() {
        demo_create_tree();
    }
}