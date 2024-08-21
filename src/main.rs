use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        thread::spawn(|| {
            thread::spawn(|| loop {
                thread::sleep(Duration::from_secs(1));
                println!("5")
            });
            thread::sleep(Duration::from_secs(1));
            std::panic!();
        })
        .join();
        thread::sleep(Duration::from_secs(100));
    })
    .join();
    println!("Hello, world!");
}
