use std::{thread, time::Duration};

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    print!("'{name}' ran for {ms}ms");
}
fn main() {
    println!("Hello, world!");
}
