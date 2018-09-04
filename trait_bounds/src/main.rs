trait Animal {
	fn make_sound(&self) -> String;
	// Default implementation ... 
	fn sleep(&self) {
		println!("ZzzzZZZzzzzzz");
	}
}

trait Eat {
	fn eating(&self);
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

impl Eat for Dog {
	fn eating(&self) {
		println!("Eating croquetas ... ");
	}
}

fn some_function<T>(t: T) where T: Animal + Eat {
	println!("{}", t.make_sound());
	t.eating();
}

fn main() {
    let nema: Dog = Dog{name: String::from("Nema")};
    println!("{:?}", nema);
    println!("{}", nema.make_sound());

    some_function(nema)
}
