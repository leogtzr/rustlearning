/*
	Explanation:

	For some lifetime 'a, the function takes two parameters (x and y), both of which are
	string slices that live at least as long as lifetime 'a.

	The function signature also tells Rust that the string slice returned from the function
	will live at least as long as lifetime 'a.

*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn main() {
	
	// string1 and string2 now have different lifetimes. This code won't compile.
	let string1 = String::from("long string is long");
	let result;
	{
		let string2 = String::from("abcd");
		result = longest(string1.as_str(), string2.as_str());
		println!("{}", result);
	}
}
