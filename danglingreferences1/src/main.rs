fn main() {
	// We may be referencing a location in memory that may have given to someone else, by freeing some memory
	// while preserving a pointer to that memory.

	// The way to fix this code is with lifetimes.	
    let reference_to_nothing = dangle();
}

// The following code fails at compile time:
fn dangle() -> &String {				// Returns a reference to a String.
	// "s" will be deallocated.
	let s: String = "Holis";
	&s
}						// Here, "s" goes out of scope.
						// Danger and dangling!
