// A watch channel is a single-producer, multiple-consumer channel typically used to monitor
// state changes of an object.
// One example is airline. When your flight can be on time, delayed, or even canceled, your
// flight status is something that must be constantly monitored. That's why airlines created
// apps that set up watch channels between you and your flight.

// For example, the Tokio documentation tells us that the consumers of a watch channel are not
// guaranteed to receive every message sent through the channel. This is because a watch channel
// only sends the most recent message, which makes sense because, if we're monitoring our flight
// status, we only care about the most up-to-date information. So if two messages are sent one
// after the other and the consumer has yet to receive either message, the watch channel will
// drop the older message and only send the newer message to the consumer.
// When you create a watch channel, you pass it the object that you want to watch. In return,
// you get a transmitter and a receiver for that channel. For the receiver, you can use the
// changed method to check if a new state of the object has been sent. To continuously wait for
// new messages, you need to place this call within a while loop. After a new state has been sent,
// you can call the RX borrow to access the message. On the transmitter side, you use the send
// method to send an updated version of the object you are watching through the channel.

// For example, if you are watching a config object and one of its values were updated,
// you should send the newly updated config object through the channel. Now that you have a
// solid understanding of how to use watch channels, feel free to monitor state changes in
// everything from flight statuses to config changes.

use tokio::time::{Duration, sleep};
use tokio::sync::watch;

pub fn watch_channel() {
    let (tx, mut rx) = watch::channel("hello".to_string());
    let _rx2= tx.subscribe();

    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        tx.send("world".to_string()).unwrap();
    });

    tokio::spawn(async move {
        while let Some(message) = rx.changed().await {
            println!("received = {:?}", message);
        }
    });
}