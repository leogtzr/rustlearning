fn main() {
    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {			// s is a reference to a String.
	s.len()					/*	s goes out of scope but since it doesn't OWN the variable, nothing happens */
}