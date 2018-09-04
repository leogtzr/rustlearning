fn main() {
	let s = String::from("hello world");
	let hello = &s[0..5];
	let world = &s[6..11];
	
	println!("{}", s);
	println!("{}", hello);
	println!("{}", world);
	
	let x = String::from("hello");
	let slice = &x[..2];
	println!("{}", slice);
	
	let name = String::from("Leonardo");
	let last_3 = &name[3..];

	println!("{}", last_3);
}
