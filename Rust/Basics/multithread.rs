// This is an example of multithreading programming in Rust

use std::thread;

fn main() {
    // immutable
    let greeting = "Hello";

    // mutable
    let mut threads = Vec::new();

    for num in 0..10 {
        threads.push(thread::spawn(move || {
            println!("{} from thread number {}", greeting, num);
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
