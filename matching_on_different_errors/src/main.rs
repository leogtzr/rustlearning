use std::fs::File;
use std::io::ErrorKind;

fn main() {
	let f = File::open("hello.txt");
	let f = match f {
		Ok(file) => file,
		// The arm will be executed if the condition is met.
		Err(ref error) if error.kind() == ErrorKind::NotFound => {
			match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => {
					panic!("Tired to create file but there was a problem: {:?}", error)
				},
			}
		},
		Err(error) => {
			panic!("There was a problem opening the file: {:?}", error)
		}
	};
}
