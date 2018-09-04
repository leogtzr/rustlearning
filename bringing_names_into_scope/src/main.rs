pub mod a {
	pub mod series {
		pub mod of {
			pub fn nested_modules() {
				println!("Hello world");
			}
		}
	}
}

/*
	The "use" keyword allow us to refer to a module but in a shorter way.
	It also brings it into scope (it does not bring children of modules into scope).
*/
use a::series::of;
use a::series::of::nested_modules;

fn main() {
	of::nested_modules();
	nested_modules();
}
