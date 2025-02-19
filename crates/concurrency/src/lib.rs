pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod threads;
pub mod message_passing;
pub mod async_tokio;
pub mod shared_state;

pub fn execute() {
    threads::move_closure::demo_move_closure();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_execute() {
        execute();
    }
}
