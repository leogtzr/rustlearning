#[derive(Debug)]
struct ImportantExcerpt<'a> {
	part: &'a str,
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