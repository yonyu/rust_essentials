use std::{future::Future, time::Duration};
use trpl::{self, Either};

fn exercise_timeout() {

    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;  // 5
            "I finished"
        };

        match timeout(slow, Duration::from_millis(10)).await {  // 2
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout<F: Future> (
    future_to_try: F,
    max_time: Duration
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeout() {
        exercise_timeout();
    }
}