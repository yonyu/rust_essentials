pub mod threads;
pub mod message_passing;
pub mod async_tokio;
pub mod shared_state;


fn main() {
    threads::fork_join::thread_creation_join();
    threads::thread_scope::exercise_thread_scope();
    threads::move_closure::demo_move_closure();
    threads::nested_threads::exercise_nested_threads();

    message_passing::channel_value_ownership::value_ownership();
    message_passing::multiple_producers::multiple_producers();
    message_passing::multiple_producers::multiple_producers_multiple_messages();

    shared_state::mutex_creation::demo_mutex();
    shared_state::mutex_share::share_mutex();
}

