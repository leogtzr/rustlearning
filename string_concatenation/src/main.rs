fn main() {
	let s1 = String::from("Hello ,");
	let s2 = String::from("world!");
	let s3 = s1 + &s2;				// s1 has been moved here ... can no longer be used.
	
	println!("{}", s3);
}
