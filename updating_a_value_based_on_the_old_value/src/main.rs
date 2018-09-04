use std::collections::HashMap;

fn main() {
	let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
    	// This returns a &mut V, a mutable reference.
    	let count = map.entry(word).or_insert(0);
    	println!("{:?}", word);
    	*count += 1;
    }

    println!("{:?}", map);
}
