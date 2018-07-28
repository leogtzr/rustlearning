enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
	match coin {
		/*
			match arms:
		*/
		Coin::Penny => {
			println!("Lucky penny!");
			1
		},
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter => 25,
	}
}

fn main() {

	let coin: Coin = Coin::Penny;
	println!("{:?}", value_in_cents(coin));

}
