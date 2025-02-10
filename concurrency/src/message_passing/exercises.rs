use std::{sync::mpsc, thread};

pub fn create_channel() {
    // the channel function returns a tuple of two values, the first value is the sender 
    // and the second value is the receiver
    let (tx, rx) = mpsc::channel::<String>();

    // Using thread::spawn to create a new thread and then using move to move ownership of 
    // tx into the new thread
        thread::spawn(move || {
        let val = String::from("Hello from the other sid");
        tx.send(val).unwrap();
    });

    // The recv method blocks the main thread until a value is sent down the channel
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_channel() {
        create_channel();
    }
}