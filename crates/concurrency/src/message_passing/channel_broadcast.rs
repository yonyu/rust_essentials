// Broadcast channel:  A broadcast channel is a multiple-producer, multiple-consumer channel.
// One example is group chats. In a group chat, each person is both a producer and a consumer.
// In Rust terms, this means that each person has access to a transmitter and receiver for the
// broadcast channel.

// One thing that is different about a broadcast channel is that receivers are created by
// calling subscribe on the transmitter. That said, every message sent by a transmitter will
// be sent to every receiver.

use tokio::sync::broadcast;

async fn broadcast_channel() {
    let (tx, mut _rx) = broadcast::channel(16); //buffer size of 16

    let tx_clone_1 = tx.clone();
    let tx_clone_2 = tx_clone_1.clone();
    let tx_clone_3 = tx_clone_1.clone();

    let mut rx_clone_1 = tx_clone_1.subscribe();
    let mut rx_clone_2 = tx_clone_1.subscribe();
    let mut rx_clone_3 = tx_clone_2.subscribe();

    tokio::spawn(async move {
        tx.send(10).unwrap();
        tx.send(20).unwrap();
        tx.send(30).unwrap();
    });

    // tokio::spawn(async move {
    //     assert_eq!(rx1.recv().await.unwrap(), 10);
    //     assert_eq!(rx1.recv().await.unwrap(), 20);
    //     assert_eq!(rx1.recv().await.unwrap(), 30);
    // });
    //
    // tokio::spawn(async move {
    //     assert_eq!(rx2.recv().await.unwrap(), 10);
    //     assert_eq!(rx2.recv().await.unwrap(), 20);
    //     assert_eq!(rx2.recv().await.unwrap(), 30);
    // });
}
