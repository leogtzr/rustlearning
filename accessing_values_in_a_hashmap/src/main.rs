use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("leo"), 27);
    scores.insert(String::from("mary"), 30);

    let team = String::from("leo");
    let score: Option<&i32> = scores.get(&team);

    println!("{:?}", score);
}
