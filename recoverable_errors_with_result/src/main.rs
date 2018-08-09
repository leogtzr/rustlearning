use std::fs::File;

fn main() {
	let f = File::open("hello");
	let f = match f {
		Ok(file) => file,
		Err(error) => {
			panic!("There was a problem opening the file: {:?}", error);
		},
	};
}