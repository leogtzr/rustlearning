use std::fmt::Display;

#[derive(Debug)]
struct Pair<T> {
	x: T,
	y: T,
}

impl<T> Pair<T> {
	fn new(x: T, y: T) -> Self {
		Self {
			x,
			y,
		}
	}
}

// cmp_display function can be invoked only when T is (implements)
// Display AND PartialOrd ... 
impl<T: Display + PartialOrd> Pair<T> {
	fn cmp_display(&self) {
		if self.x >= self.y {
			println!("The largest number is x = {}", self.x);
		} else {
			println!("The largest number is x = {}", self.y);
		}
	}
}

// impl<T: Display> ToString for T {
// 	// --snip--
// }

fn main() {
	let pair = Pair{x: 2, y: 6};
	pair.cmp_display();
}