use std::thread;
use std::time::Duration;

fn main() {

    println!("Hello ... ");
    thread::sleep(Duration::from_secs(2));
    println!("World");

}
