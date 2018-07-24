fn main() {
    let reference_to_nothing = dangle();
}

// The following code fails at compile time:
fn dangle() -> &String {
	let s: String = "Holis";
	&s
}