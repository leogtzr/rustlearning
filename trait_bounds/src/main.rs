trait Animal {
	fn make_sound(&self) -> String;
	// Default implementation ... 
	fn sleep(&self) {
		println!("ZzzzZZZzzzzzz");
	}
}

#[derive(Debug)]
struct Dog {
	name: String,
}

impl Animal for Dog {
	fn make_sound(&self) -> String {
		format!("{}: woof woof", self.name)
	}
}

fn main() {
    let nema: Dog = Dog{name: String::from("Nema")};
    println!("{:?}", nema);
    println!("{}", nema.make_sound());
}
