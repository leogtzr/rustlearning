extern crate media_aggregator;

use media_aggregator::Tweet;

fn main() {
	println!("{:?}", "Holaaa!");

	/*
	pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: String,
	pub retweet: String,
}
*/

	let tweet: Tweet = Tweet {
		username: String::from("Holis"),
		content: String::from("Hello"),
		reply: String::from("hjdsdfsdf"),
		retweet: String::from("sdfsdf"),
	};
	println!("{:?}", tweet);
}