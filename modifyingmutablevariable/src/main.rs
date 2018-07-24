fn main() {
    let mut s: String = String::from("hello");
    change(&mut s);

    println!("{}", s);
}

fn change(s: &mut String) {
	s.push_str("Holis");
}