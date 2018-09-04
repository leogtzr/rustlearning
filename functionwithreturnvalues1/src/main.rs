fn four() -> i32 {
	return 4;
}

fn five() -> i32 {
	four() + 1
}

fn plus_one(x: i32) -> i32 {
	x + 1
}

fn main() {
	println!("{:?}", five());
	println!("{}", plus_one(3));
}
