use std::thread;
use std::time::Duration;

fn main() {

    let expensive_closure = |num: i32| -> i32 {
        println!("Calculating slowly ... ");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let n = expensive_closure(34);

    println!("Value: {}", n);

}
