fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn main() {
	let s = String::from("apple");
	let (first, remainder) = car_cdr(&s);
	
	match first as &str {
		"a" | "e" | "i" | "o" | "u" => {
			println!("{}", format!("{}-hay", s));
		},
		_ => {
			println!("{}", format!("{}-{}ay", remainder, first));
		}
	}
}
