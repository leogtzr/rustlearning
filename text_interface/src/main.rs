use std::io;
use std::collections::HashMap;

fn main() {
	let mut user_input = String::new();
	// @PENDING
	// let mut people = HashMap::new();

	loop {
		println!("Input: ");
		match io::stdin().read_line(&mut user_input) {
	    	Ok(_) => {
				let vec = user_input.split_whitespace().collect::<Vec<&str>>();
				for w in vec {
					println!("Word: {}", w);
				}
	    	}
		    Err(_) => continue,
		}
		
		if user_input.trim() == "quit" {
			break;
		}	
	}
}
