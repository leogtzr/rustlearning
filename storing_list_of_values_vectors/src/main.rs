fn main() {
	//let v: Vec<i32> = Vec::new();
	// Creating a new vector containing values.
	// the variable has to be mutable in order to change.
	let mut v = vec![1, 2, 3];
	v.push(2);
	v.push(4);
	v.push(5);
	println!("{:?}", v);
	
	let third: &i32 = &v[2];
	println!("Third: {}", third);
}
