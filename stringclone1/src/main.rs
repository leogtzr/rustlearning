fn main() {
	let s1 = String::from("Leo");
	let s2 = s1.clone();			// Heap data gets copied over and might be expensive.

	println!("{:?}", s1);
	println!("{:?}", s2);
}