# Fork-Join Parallelism Pattern

Fork-Join parallelism is a programming pattern that allows you to divide a task into smaller subtasks
and execute them concurrently. Once all subtasks are completed, the results are combined to produce
the final result. This pattern is useful when you have a large task that can be divided into smaller
independent tasks.

In this pattern, to fork is to start a new thread, and to join a thread is to wait for it to finish.

Fork-join parallelism is attractive for a few reasons:
- It’s dead simple. Fork-join is easy to implement, and Rust makes it easy to get
right.
- It avoids bottlenecks. There’s no locking of shared resources in fork-join. The
only time any thread has to wait for another is at the end. In the meantime, each
thread can run freely. This helps keep task-switching overhead low.
- The performance math is straightforward. In the best case, by starting four
threads, we can finish our work in a quarter of the time. Be aware that we might
not be able to distribute the work evenly across all threads. Another reason for
caution is that sometimes fork-join programs must spend some time after the threads
join combining the results computed by the threads. That is, isolating the tasks
completely may make some extra work. Still, apart from those two things, any
CPU-bound program with isolated units of work can expect a significant boost.
- It’s easy to reason about program correctness. A fork-join program is deterministic
as long as the threads are really isolated, like the compute threads in the Mandelbrot
program. The program always produces the same result, regardless of
variations in thread speed. It’s a concurrency model without race conditions.

The main disadvantage of fork-join is that it requires isolated units of work. That
is, fork-join parallelism is not always easy to apply.

In Rust, besides using thread spawn and join, you can implement Fork-Join parallelism 
using the `rayon` crate. The `rayon` crate provides a high-level API for parallelism 
that allows you to write parallel code without dealing with low-level thread management.

Here's an example of implementing Fork-Join parallelism using `rayon`:

```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum: i32 = data.par_iter().map(|&x| x).sum();

    println!("Sum: {}", sum);
}
```