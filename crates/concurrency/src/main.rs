pub mod threads;
pub mod message_passing;
pub mod async_tokio;
pub mod channels;


fn main() {
    threads::fork_join::thread_creation_join();
    threads::thread_scope::exercise_thread_scope();
    threads::move_closure::demo_move_closure();
    threads::nested_threads::exercise_nested_threads();
}

