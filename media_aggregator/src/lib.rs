pub trait Summary {
	fn summary(&self) -> String {
		String::from("(Read more ... )")
	}
}

#[derive(Debug)]
pub struct NewsArticle {
	pub headline: String,
	pub location: String, 
	pub author: String,
	pub content: String,
}

impl Summary for NewsArticle {
	fn summary(&self) -> String {
		format!("{} by {} ({})", self.headline, self.author, self.location)
	}
}

#[derive(Debug)]
pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: String,
	pub retweet: String,
}

impl Summary for Tweet {
    fn summary(&self) -> String {
    	format!("{}:{}", self.username, self.content)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
