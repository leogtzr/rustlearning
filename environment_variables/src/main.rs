use std::env;

fn main() {
    const KEY: &str = "HOME";
    
    match env::var(KEY) {
        Ok(value) => println!("Key: {}, value: {}", KEY, value),
        Err(e) => println!("couldn't interpret key: {} -> {}", KEY, e),
    }

}
