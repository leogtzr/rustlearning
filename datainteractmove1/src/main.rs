fn main() {
	let x = 5;
	let y = x;

	println!("{:?}", x);
	println!("{:?}", y);

	let s1 = String::from("hello");

	// s1 was moved into s2.
	let s2 = s1;

	// println!("{:?}", s1);
	/*
		Explanation of the previous error: 
		Rust considers s1 to no longer be valid and, therefore, Rust doesn't need to free anything
		when s1 goes out of scope.

		It is not called a shallow copy (copying the pointer, length and capacity), it is considered a MOVE.
	*/
	println!("{:?}", s2);
}
