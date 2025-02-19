//use std::io::Read;
use std::sync::mpsc;
//use std::sync::mpsc::channel;
use std::thread;

// two producers and a single receiver, the receiving order is not guaranteed
pub fn multiple_producers() {
    let (tx, rx) = mpsc::channel();
    // clone the transmitter so that there are two producers
    // and a single receiver that can receive messages from both producers
    let tx1 = tx.clone();

    thread::spawn(move || {
        let data = b"Hello, World!";
        tx1.send(data.to_vec()).unwrap();
    });

    thread::spawn(move || {
        let data = b"Goodbye, world!";
        tx.send(data.to_vec()).unwrap();

    });

    // receive one message from either producer1 1 or producer 2
    let recv1 = rx.recv().unwrap();
    println!("{:?}", recv1);
    // receive the other message
    let recv2 = rx.recv().unwrap();
    println!("{:?}", recv2);

    // for received in rx {
    //     println!("{:?}", received);
    // }
}

// Sending multiple messages from multiple producers
pub fn multiple_producers_multiple_messages() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let data = vec!["Hello", "from", "thread", "1"];
        for msg in data {
            tx.send(msg.to_string()).unwrap();
            thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    thread::spawn(move || {
        let data = vec!["Greetings", "by", "second", "producer"];
        for msg in data {
            tx1.send(msg.to_string()).unwrap();
        }
    });

    for received in rx {
        println!("Got: {:?}", received);
        thread::sleep(std::time::Duration::from_millis(10));
    }
}