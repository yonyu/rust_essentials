use std::{future::Future, pin::{Pin, pin}, time::Duration};

use trpl;

// Creating an async channel and assigning the two halves to tx and rx
pub fn basic_passing() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        // Notice that we do not have to spawn a separate thread or even a task;
        // we merely need to await the rx.recv call.
        let received = rx.recv().await.unwrap();

        println!("Got: {received}");
    });
}

// this version has two flaws:
// 1. They arrive all at once, two seconds (2,000 milliseconds) after we start the program.
// 2. This program also never exits! Instead, it waits forever for new messages.
pub fn passing_messages_with_two_flaws() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    });
}

// This program never exits! Instead, it waits forever for new messages.
pub fn passing_messages_one_flaw() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}

pub fn passing_messages() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}

pub fn passing_messages_two_producers() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // can also use trpl::join!(tx1_fut, tx_fut, rx_fut);
        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    });
}

// Working with any number of producers
// Using Pin and Box::pin to make the Vec type check, using the heap memory to store futures
pub fn passing_messages_any_producers_heap() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(tx1_fut), Box::pin(tx_fut), Box::pin(rx_fut)];

        trpl::join_all(futures).await;
    });
}


// Working with any number of producers
// Using the std::pin::pin macro to avoid unnecessary heap allocations
pub fn passing_messages_any_producers() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, tx_fut, rx_fut];

        // join_all() can deal with any number of futures that have the same type
        trpl::join_all(futures).await;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_passing() {
        basic_passing();
    }

    #[test]
    fn test_passing_messages() {
        passing_messages();
    }

    #[test]
    fn test_passing_messages_two_producers() {
        passing_messages_two_producers();
    }

    #[test]
    fn test_passing_messages_any_producers_heap() {
        passing_messages_any_producers_heap();
    }

    
    #[test]
    fn test_passing_messages_any_producers() {
        passing_messages_any_producers();
    }

}
