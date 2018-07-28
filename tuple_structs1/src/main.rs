#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
	let black: Color = Color(0, 0, 0);
	let point: Point = Point(0, 0, 0);
	println!("{:?}", black);
	println!("{:?}", point);

	let x: i32 = point.0;

	println!("{:?}", point);
	println!("{:?}", x);

}

