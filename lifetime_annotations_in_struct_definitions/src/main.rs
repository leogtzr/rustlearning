#[derive(Debug)]
struct ImportantExcerpt<'a> {
	part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
	fn level(&self) -> i32 {
		3
	}
}

impl<'a> ImportantExcerpt<'a> {
	fn announce_and_return_part(&self, announcement: &str) -> &str {
		println!("Attention please: {}", announcement);
		self.part    				// the return lifetime is not specified
									// but based on the rules self's elision
									// lifetime is assigned here.
	}
}

fn main() {
    let novel = String::from("Call me Leo. Some years ago ...");
    let first_sentence = novel.
    	split(".").
    	next().
    	expect("Could not find a '.'");
    let i = ImportantExcerpt{part: first_sentence};

    // This compile because the reference that ImportantExcerpt holds can't
    // outlive the reference it holds (part).
}