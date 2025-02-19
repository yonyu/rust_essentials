use std::thread;
use std::time::Duration;

// exercise #1: create a new thread with std::thread::spawn
// and wait for it to finish with handle.join().
//
// This pattern is called fork-join parallelism. To fork is to start a new thread, 
// and to join a thread is to wait for it to finish.
pub fn thread_creation_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // wait for the spawned thread to finish. this is necessary because the spawned thread is 
    // a daemon thread and will be terminated when the main thread ends
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        thread_creation_join();
    }
}