fn foo(x: i32, y: i32) -> i32 {
	println!("The value of x is: {}", x);
	println!("The value of y is: {}", y);
	return x;
}

fn main() {
	println!("{:?}", foo(3, 5));
}