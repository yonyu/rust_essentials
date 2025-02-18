use std::thread;

#[allow(unused_assignments)]
pub fn exercise_nested_threads() {
    let mut n = 1;
    let t = thread::spawn(move || {
        n = n + 1;
        // Spawn a nested thread and return its JoinHandle
        thread::spawn(move || {
            n = n + 1;
        })
    });

    n = n + 1;
    // Join the outer thread and then the nested thread
    //
    // t.join().unwrap() returns the result of the outer thread's execution. Specifically, 
    // it returns the JoinHandle of the nested thread that was spawned within the outer 
    // thread. This JoinHandle is then used to join the nested thread.
    t.join().unwrap().join().unwrap();
    
    println!("{n}");

    // equivalent to the following code
    {
        // Create a new thread
        let t = thread::spawn(move || {
            n = n + 1;
            // Create a new thread
            let t = thread::spawn(move || {
                n = n + 1;
            });
            // Wait for the nested thread to finish
            t.join().unwrap();
        });
        // Wait for the thread to finish
        t.join().unwrap();
    }
}