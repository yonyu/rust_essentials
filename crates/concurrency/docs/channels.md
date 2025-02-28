# What is a channel
Channels are a way of passing data from one thread to another.

A channel is like a pipeline between threads.

- One thread can send data into the channel
- another thread can read data from the channel without contention.

So it's like message queuing in memory.

Creating a channel as follows:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
```

channels return a tuple, so `tx` is the sending half and `rx` is the receiving half.

