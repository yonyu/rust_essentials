use std::thread;
use std::time::Duration;

pub fn demo_move_closure() {
    let v = vec![1, 2, 3];

    // Use the move keyword to force the closure to take ownership of v
    let handle = thread::spawn( move || {
        println!("Here's a vector: {:?}", v);
        thread::sleep(Duration::from_millis(1000));
    });

    // drop(v); // try to drop v here and see what happens: error[E0382]: use of moved value: `v`
    
    handle.join().unwrap();
}