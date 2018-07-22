fn main() {
	//let tup: (i32, i32, i32) = (1, 2, 3);
	let tup = (1, 2, 3);
	println!("{:?}", tup.0);
	println!("{:?}", tup.1);
	println!("{:?}", tup.2);

	let tup2: (i8, f64, bool) = (4i8, 4.5, true);
	println!("{:?}", tup2.0);

	if tup2.2 {
		println!("{:?}", "Ok alv");
	}
}
