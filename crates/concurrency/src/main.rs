pub mod threads;
pub mod message_passing;
pub mod async_await;
pub mod shared_state;

use async_await::exercise1;
use trpl::Either;

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

    let mut args: Vec<String> = std::env::args().collect();
    args.push(String::from("https://google.com"));
    args.push(String::from("https://msn.com"));

    trpl::run(async {

        let title_fut_1 = exercise1::page_title_url(&args[1]);
        let title_fut_2 = exercise1::page_title_url(&args[2]);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

