#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
}

#[derive(Debug)]
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}", state);
			25
		},
	}
}

fn main() {
	let x: u32 = value_in_cents(Coin::Quarter(UsState::Alaska));
	println!("{:?}", x);

	let mut count = 0;

	if let Coin::Quarter(state) = Coin::Quarter(UsState::Alabama) {
		println!("The state is: {:?}", state);
	} else {
		count += 1;
	}

	println!("Count: {}", count);
}
