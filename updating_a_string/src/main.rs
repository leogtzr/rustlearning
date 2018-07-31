fn main() {
	let mut s = String::from("foo");
	println!("{}", s);
	let s2 = "bar";
	s.push_str(s2);
	println!("Content: {}", s);
	s.push('_');
	println!("Content: {}", s);
}
