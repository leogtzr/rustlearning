#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {					// &mut self to change the parameters.
		self.width * self.height
	}

	fn can_hold(&self, r1: &Rectangle) -> bool {
		self.width > r1.width && self.height > r1.height
	}
}

fn main() {
    
	let rect1 = Rectangle{width: 30, height: 50};
	let rect2 = Rectangle{width: 10, height: 40};
	let rect3 = Rectangle{width: 60, height: 45};

	println!("Can react1 hold rect2: {}", rect1.can_hold(&rect2));
	println!("Can react1 hold rect3: {}", rect1.can_hold(&rect3));

	println!("{:?}", rect3.area());

}
