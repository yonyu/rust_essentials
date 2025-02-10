use tokio;
//use tokio::task::JoinHandle;

async fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tokio::main]
async fn join_await() {
    // tokio::spawn returns a JoinHandle, which is a future representing the output of the spawned task
    let join_handle = tokio::spawn(hello("world"));

    let result = join_handle.await.unwrap();
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_await() {
        join_await();
    }

    //#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    #[tokio::test]
    async fn test_hello() {
        let result = hello("world").await;
        assert_eq!(result, "Hello, world!");
    }
}