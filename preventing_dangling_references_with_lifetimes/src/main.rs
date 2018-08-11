fn main() {
    {
    	// The data (x) has a longer live than the reference (r).
    	// This code is valid:
    	let x = 5;
    	let r = &x;
    	println!("r: {}", r);
    }
}
