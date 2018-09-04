fn main() {
    let s: String = String::from("hello");
    change(&s);
}

fn change(s: &String) {
	s.push_str("Holis");
}