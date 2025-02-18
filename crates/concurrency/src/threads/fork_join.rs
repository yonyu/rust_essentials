use std::thread;
use std::time::Duration;

// exercise #1: create a new thread with std::thread::spawn
// and wait for it to finish with handle.join().
//
// This pattern is called fork-join parallelism. To fork is to start a new thread, 
// and to join a thread is to wait for it to finish.
//
// Fork-join parallelism is attractive for a few reasons:
// • It’s dead simple. Fork-join is easy to implement, and Rust makes it easy to get
// right.
// • It avoids bottlenecks. There’s no locking of shared resources in fork-join. The
// only time any thread has to wait for another is at the end. In the meantime, each
// thread can run freely. This helps keep task-switching overhead low.
// • The performance math is straightforward. In the best case, by starting four
// threads, we can finish our work in a quarter of the time. Be aware that we might 
// not be able to distribute the work evenly across all threads. Another reason for 
// caution is that sometimes fork-join programs must spend some time after the threads 
// join combining the results computed by the threads. That is, isolating the tasks 
// completely may make some extra work. Still, apart from those two things, any 
// CPU-bound program with isolated units of work can expect a significant boost.
// • It’s easy to reason about program correctness. A fork-join program is deterministic
// as long as the threads are really isolated, like the compute threads in the Mandelbrot
// program. The program always produces the same result, regardless of
// variations in thread speed. It’s a concurrency model without race conditions.
//
// The main disadvantage of fork-join is that it requires isolated units of work. That
// is, fork-join parallelism is not always easy to apply.
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