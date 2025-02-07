use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    // A Node owns its children, and we want to share that ownership with variables
    // so we can access each Node in the tree direcly, so we define Vec<Rc<Node>>.
    // We also want to modify which nodes are children of other nodes, so we use RefCell
    // around Vec<Rc<Node>>, RefCell<Vec<Rc<Node>>>.
    chidren: RefCell<Vec<Rc<Node>>>,
}

pub fn demo_create_tree() {
    let leaf = Rc::new(Node {
        value: 3,
        chidren: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        chidren: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Some(Rc::clone(&branch));

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}