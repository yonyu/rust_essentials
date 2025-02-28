use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::time::{Duration, sleep};

async fn person (semaphore: std::sync::Arc<tokio::sync::Semaphore>, name: String) {
    println!("{} is waiting for the teller", name);
    teller(semaphore, name).await;

}

async fn teller(semaphore: Arc<Semaphore>, customer: String) {
    let permit = semaphore.acquire().await.unwrap();

    sleep(Duration::from_secs(2)).await;
    println!("\n{} is being served by the teller", customer);
    sleep(Duration::from_secs(5)).await;
    println!("\n{} is now leaving the teller", customer);

    drop(permit);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_bank_teller() {
        let semaphore = Arc::new(Semaphore::new(1));
        let semaphore1 = Arc::clone(&semaphore);
        let semaphore2 = Arc::clone(&semaphore);
        let semaphore3 = Arc::clone(&semaphore);

        let person1 = person(semaphore1, "Alice".to_string());
        let person2 = person(semaphore2, "Bob".to_string());
        let person3 = person(semaphore3, "Carol".to_string());

        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            tokio::join!(person1, person2, person3);
        });
    }

    #[tokio::test]
    async fn test_bank_teller_async() {
        let num_of_tellers = 4;
        let semaphore = Semaphore::new(num_of_tellers);
        let semaphore_arc = Arc::new(semaphore);

        let mut people_handles = Vec::new();
        for num in 0..10 {
            people_handles.push(tokio::spawn(person(Arc::clone(&semaphore_arc), format!("Customer {}", num))));
        }

        for handle in people_handles {
            handle.await.unwrap();
        }
    }
}