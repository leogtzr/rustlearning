#[derive(Debug)]
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

impl<T> Point<T> {
	fn y(&self) -> &T {
		&self.y
	}
}

impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

fn main() {
	let p = Point{x: 12.3, y: 45.3};
	println!("{:?}", p);

	println!("{:?}", p.x());
	println!("{:?}", p.y());
}