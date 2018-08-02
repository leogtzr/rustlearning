use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // The original value has been overriden:
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}
