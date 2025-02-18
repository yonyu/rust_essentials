use std::thread;

// thread::scope(() creates a scope for spawning scoped threads.
// 
// The function passed to scope will be provided a Scope object, through which scoped threads
// can be spawned.
// 
// Unlike non-scoped threads, scoped threads can borrow non-'static data, as the scope 
// guarantees all threads will be joined at the end of the scope.
// 
// All threads spawned within the scope that haven’t been manually joined will be automatically 
// joined before this function returns.
pub fn exercise_thread_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            // We can borrow `a` here.
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            // We can even mutably borrow `x` here,
            // because no other threads are using it.
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}