use std::time::Duration;
use trpl;

pub fn run_counting() {
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in  1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}

pub fn run_counting_join() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in  1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // the task handle is a future, use await to wait until the task is complete
        handle.await.unwrap();
    });
}

// Don't spawn a new operating system thread.
// Here we create two async blocks, each of them returns a future.
// We await both futures to finish by using trpl::join().
pub fn run_counting_join_2() {
    trpl::run(async {
        // The two tasks in the run_counting_join_2 function do not run
        // in different threads. They run concurrently within the same
        // thread using asynchronous programming.

        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in  1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // By using trpl::join, the function ensures that both tasks
        // run concurrently without blocking each other. The await on
        // the join function waits for both tasks to finish before the
        // run_counting_join_2 function completes.
        //
        // We do not await fut1 and fut2, but instead the new future 
        // produced by trpl::join. 
        // We ignore the output, because it is just a tuple with two
        // unit values in it.
        trpl::join(fut1, fut2).await;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_counting() {
        run_counting();
    }

    #[test]
    fn test_run_counting_join(){
        run_counting_join();
    }

    #[test]
    fn test_run_counting_join_2(){
        run_counting_join_2();
    }
}