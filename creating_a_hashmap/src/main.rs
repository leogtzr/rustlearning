use std::collections::HashMap;

fn main() {
    let mut values: HashMap<&str, &str> = HashMap::new();
    values.insert("Leo", "leogtzr");

    println!("{:?}", values);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    println!("{:?}", scores);

}