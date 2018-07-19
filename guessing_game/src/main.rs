use std::io;

fn main() {
	
	println!("Guess the number!");
	println!("Please input your guess.");
	
	// :: an associated function ... 
	// Some languages call this static methods ... 
	let mut guess = String::new();

	io::stdin().read_line(&mut guess).expect("Failed to read line");
	//io::stdin().read_line(&mut guess).expect("Failed to read line");
	println!("You guessed: {}", guess);


}
