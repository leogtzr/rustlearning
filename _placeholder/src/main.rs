fn main() {
	let some_u8_value: u8 = 1u8;

	match some_u8_value {
		1 => println!("Uno"),
		2 => println!("Dos"),
		_ => (),
	}
}
