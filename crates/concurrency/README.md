## Concurrent Programming in Rust

Concurrent programming is a programming technique that allows multiple tasks to
run independently. Rust provides powerful abstractions for writing concurrent 
programs that are safe and efficient. 

This section covers the following topics:
- [Fork-Join Parallelism](docs/fork-join.md)
> Fork-Join parallelism is a programming pattern that allows you to divide a task 
> into smaller subtasks and execute them concurrently. Once all subtasks are 
> completed, the results are combined to produce the final result.

- Channels
> Rust implements channels to accomplish message passing between threads.

- Shared Mutable State
