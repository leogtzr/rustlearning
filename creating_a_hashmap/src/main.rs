use std::collections::HashMap;

fn main() {
    let mut values: HashMap<&str, &str> = HashMap::new();
    values.insert("Leo", "leogtzr");

    println!("{:?}", values);
}