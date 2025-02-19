use std::sync::Mutex;

pub fn demo_mutex() {
    // create a mutex
    let m = Mutex::new(5);

    // use a block to ensure the mutex is released when the block ends
    {
        // lock the mutex to access the value, unwrap returns MutexGuard
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}