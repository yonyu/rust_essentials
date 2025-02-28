use std::sync::Arc;
//use std::thread::sleep;
//use std::time::Duration;

use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};

pub async fn person(remove_arc: Arc<Mutex<i32>>, name: String, new_channel: i32) {
    // request access to the remote
    let mut real_remote = remove_arc.lock().await;

    // change the channel
    *real_remote = new_channel;
    println!("{} changed the channel", name, );
    println!("Watching channel {}", new_channel);

    // simulate watching TV
    sleep(Duration::from_secs(5)).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_watch_tv() {
        let remote = Arc::new(Mutex::new(0));
        let remote1 = Arc::clone(&remote);
        let remote2 = Arc::clone(&remote);
        let remote3 = Arc::clone(&remote);

        let person1 = person(remote1, "Alice".to_string(), 11);
        let person2 = person(remote2, "Bob".to_string(), 32);
        let person3 = person(remote3, "Carol".to_string(), 43);

        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut people_handles = Vec::new();
            //tokio::join!(person1, person2, person3);
            people_handles.push(tokio::spawn(person1));
            people_handles.push(tokio::spawn(person2));
            people_handles.push(tokio::spawn(person3));

            for handle in people_handles {
                handle.await.unwrap();
            }
        });
    }

    #[tokio::test]
    async fn test_watch_tv_async() {
        let tv_channel = 10;
        let remote = Mutex::new(tv_channel);
        let remote_arc = Arc::new(remote);

        let mut people_handles = Vec::new();

        people_handles.push(tokio::spawn(person(Arc::clone(&remote_arc), "Alice".to_string(), 11)));
        people_handles.push(tokio::spawn(person(Arc::clone(&remote_arc), "Bob".to_string(), 32)));
        people_handles.push(tokio::spawn(person(Arc::clone(&remote_arc), "Carol".to_string(), 43)));

        for handle in people_handles {
            handle.await.unwrap();
        }
    }
}