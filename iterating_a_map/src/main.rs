use std::collections::HashMap;

fn main() {

	let mut ages: HashMap<&str, &i32> = HashMap::new();
	ages.insert("ok", &27);

	println!("{:?}", ages);

	for (k, v) in ages {
		println!("{:?} => {:?}", k, v);
	}

}