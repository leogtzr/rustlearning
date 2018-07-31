fn main() {
	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	// Much easier to read and it doesn't take ownership of the arguments.
	let s = format!("{}-{}-{}", s1, s2, s3);

	println!("{}", s);
}
