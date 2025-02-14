pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod threads;
pub mod message_passing;
pub mod async_tokio;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
