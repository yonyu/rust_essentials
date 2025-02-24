//use std::sync::mpsc::Receiver;

use std::pin::pin;
use std::time::Duration;

use trpl::{self, ReceiverStream, Stream};
use trpl::StreamExt;

#[allow(dead_code)]
fn stream_from_iter() {
    trpl::run( async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}

#[allow(dead_code)]
fn stream_from_iter_with_filter() {
    trpl::run( async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}

#[allow(dead_code)]
fn print_messages() {
    trpl::run(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });
}

#[allow(dead_code)]
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}

#[allow(dead_code)]
fn get_messages_with_timeout() -> impl Stream<Item = String> {

    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Messages: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

#[allow(dead_code)]
fn print_messages_with_timeout() {
    trpl::run(async {
        let mut messages = pin!(get_messages_with_timeout().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

#[allow(dead_code)]
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();

        }
    });

    ReceiverStream::new(rx)
}

#[allow(dead_code)]
fn print_intervals() {
    trpl::run(async {
        let messages = get_messages_with_timeout().timeout(Duration::from_millis(200));
        // transform the intervals into a stream of strings
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        // make steam mutable, so that the while let loop's next calls can iterate through
        // the stream , and pin it so that is safe to do so.
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_from_iter() {
        stream_from_iter();
    }

    #[test]
    fn test_stream_from_iter_with_filter() {
        stream_from_iter_with_filter();
    }

    #[test]
    fn test_print_message() {
        print_messages();
    }

    #[test]
    fn test_print_messages_with_timeout() {
        print_messages_with_timeout();
    }

    #[test]
    fn test_print_intervals() {
        print_intervals();
    }    
}
