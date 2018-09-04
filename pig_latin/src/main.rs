fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn pig(s: &str) -> String {
	let (first, remainder) = car_cdr(s);

	let x = match first as &str {
		"a" | "e" | "i" | "o" | "u" => format!("{}-hay", s),
		_ => format!("{}-{}ay", remainder, first)
	};
	x
}

fn main() {
	let s = String::from("apple");
	println!("{}", pig(&s));
}
