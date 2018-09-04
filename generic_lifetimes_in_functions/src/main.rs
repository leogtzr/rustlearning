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
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);
}
