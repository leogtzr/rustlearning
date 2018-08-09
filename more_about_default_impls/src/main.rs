#[derive(Debug)]
struct Tweet {
	username: String,
}

trait Summary {

	fn summarize_author(&self) -> String;

	fn summary(&self) -> String {
		format!("Hello ... {}", self.summarize_author())
	}
}

fn notify<T: Summary>(item: T) {
	println!("Breaking news!: {}", item.summary());
}

impl Summary for Tweet {
	fn summarize_author(&self) -> String {
		format!("I am: {}", self.username)
	}
}

fn main() {
    let tweet: Tweet = Tweet{username: String::from("Aloooo")};
    println!("{:?}", tweet);
    println!("{}", tweet.summarize_author());
    notify(tweet);
}
