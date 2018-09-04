fn main() {
	let hello = "Hello world";
	let s = &hello[0..4];

	println!("{}", s);

	for c in hello.chars() {
		println!("{}", c);
	}

	println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

	for c in s.chars() {
		println!("{}", c);
	}
	
	println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
	
	for b in "Ninos".bytes() {
		println!("{}", b);
	}
}
