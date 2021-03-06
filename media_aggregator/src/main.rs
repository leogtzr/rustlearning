extern crate media_aggregator;

use media_aggregator::Summary;
use media_aggregator::Tweet;

fn main() {
	println!("{:?}", "Holaaa!");

	let tweet: Tweet = Tweet {
		username: String::from("Holis"),
		content: String::from("Hello"),
		reply: String::from("hjdsdfsdf"),
		retweet: String::from("sdfsdf"),
	};
	println!("{:?}", tweet);

	println!("{}", tweet.summary());
}