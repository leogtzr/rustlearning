#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn square(size: u32) -> Rectangle {
		Rectangle{height: size, width: size}
	}
}

fn main() {
    let r1: Rectangle = Rectangle::square(3);
    println!("{:?}", r1);
}
