extern crate rand;

use std::io;
// Ordering is another enum ... 
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	
	println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(1, 100);
	println!("The secret number is: {}", secret_number);

	loop {
		println!("Please input your guess.");

		// :: an associated function ... 
		// Some languages call this static methods ... 
		let mut guess = String::new();

		io::stdin().read_line(&mut guess).expect("Failed to read line");

		// trim() eliminates the new line.
		// :type -> we are anotating the variable with a type.

		let guess: i32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Enter a valid number! :@");
				continue;
			}
		};

		if guess < 1 || guess > 100 {
			println!("The number will be between 1 and 100");
			continue;
		}

		//io::stdin().read_line(&mut guess).expect("Failed to read line");
		println!("You guessed: {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}

}
