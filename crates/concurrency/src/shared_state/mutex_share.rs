//use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
//use std::thread::JoinHandle;


pub fn share_mutex() {
    // create a mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![]; //Vec::<JoinHandle<i32>>::new();
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            // lock the mutex to access the value
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

/*
// error[E0277]: `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
pub fn share_mutex_v2() {
    // create a mutex
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![]; //Vec::<JoinHandle<i32>>::new();
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        // error[E0277]: `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
        let h = thread::spawn(move || { // the trait `Send` is not implemented for `Rc<std::sync::Mutex<i32>>` note: required because it's used within this closure
            // lock the mutex to access the value
            // value (counter) moved into closure here, in previous iteration of loop
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // value (counter) borrowed here after move
}
 */

/*
pub fn share_mutex_v1() {
    // create a mutex
    let counter = Mutex::new(0);
    let mut handles = vec![]; //Vec::<JoinHandle<i32>>::new();
    for i in 0..10 {
        let h = thread::spawn(move || { // value moved into closure here, in previous iteration of loop
            // lock the mutex to access the value
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // value (counter) borrowed here after move
}
*/
