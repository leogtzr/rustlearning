struct Point<T> {
	x: T,
	y: T,
}

// Getters
impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}

	fn y(&self) -> &T {
		&self.y
	}
}

impl Point<f64> {
	fn distance_from_origin(&self) -> f64 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

fn main() {
}
