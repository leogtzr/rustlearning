fn main() {
	let a: [i32; 5] = [10, 20, 30, 40, 50];
	for element in a.iter() {
		println!("The value is: {}", element);
	}
}
