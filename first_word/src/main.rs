fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i]
		}
	}
	&s[..]
}

fn main() {
	let my_string = String::from("hello world");
	let word = first_word(&my_string[..]);
	println!("{:?}", word);

	let my_literal_string = "hello world";
	let word = first_word(&my_literal_string[..]);
	println!("{:?}", word);

	// my_literal_string is already a slice so we can use directly.
	let word = first_word(my_literal_string);
	println!("{:?}", word);

}
