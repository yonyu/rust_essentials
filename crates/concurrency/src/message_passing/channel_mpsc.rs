// A multi-producer, single-consumer channel.

use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn channel_mpsc() {
    let (mut tx, mut rx) = mpsc::channel(32);
    //let tx_arc = Arc::new(tx);

    tokio::spawn(async move {
        tx.send("Hello from the other side".to_string()).await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("Got: {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[tokio::test]
    async fn test_channel_mpsc() {
        channel_mpsc().await;
    }
}