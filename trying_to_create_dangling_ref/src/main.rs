// This code does not compile.

fn longest<'a>(x: &str, y: &str) -> &'a str {
	let result = String::from("really long string");
	result.as_str()
}

fn main() {
    
}
