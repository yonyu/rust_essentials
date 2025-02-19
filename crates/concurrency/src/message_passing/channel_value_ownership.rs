use std::sync::mpsc;
use std::thread;

pub fn value_ownership() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let value = String::from("Hello from thread 1");

        tx.send(value).unwrap();
        // once the value has been sent to another thread, that thread could modify or drop
        // it before we try to use the value again. This is why the following line would
        // result in a compile-time error:
        // println!("Sent: {}", value); // error[E0382]: borrow of moved value: `value`
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}