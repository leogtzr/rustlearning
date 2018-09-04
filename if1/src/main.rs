fn main() {
	let truth: bool = true;
	let x: i32 = if truth {
		2
	} else {
		3
	};

	println!("{:?}", x);

	let number = 3;
	if number < 3 {
		println!("Condition was true");
	} else {
		println!("Condition was false");
	}

	let number = 6;
	println!("{:?}", number);

	if number % 4 == 0 {
		println!("number is divisible by 4");
	} else if number % 3 == 0 {
		println!("number is divisible by 3");
	} else if number % 2 == 0 {
		println!("number is divisible by 2");
	} else {
		println!("number is not divisible by 4, 3 or 2");
	}
}
