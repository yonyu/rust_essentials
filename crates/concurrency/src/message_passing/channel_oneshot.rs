// By definition, a oneshot channel is a single-producer, single-consumer channel that can send one
// message from the transmitter to the receiver. Unlike it's Hollywood counterpart, the message of
// the oneshot channel does not self-destruct. Instead, the transmitter gets destroyed, which makes
// the receiver useless.

// Here's how it works. We create a oneshot channel, which gives us a transmitter and a receiver.
// We pass the transmitter to one task, which can represent the spy agency, and we pass the
// receiver to another task, which represents the spy. Once a message has been sent from the spy
// agency to the spy, the transmitter is no longer usable, which means that no more messages can
// be sent through this channel.

use std::thread;
use tokio::sync::{mpsc, oneshot};

pub fn oneshot_channel() {
    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        let val = String::from("Hello from the other side");
        tx.send(val).unwrap();
    });

    let received = rx.blocking_recv().unwrap();
    println!("Got: {}", received);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oneshot_channel() {
        oneshot_channel();
    }
}