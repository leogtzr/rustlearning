const DEFAULT_AGE: u32 = 25;

fn main() {
    let age: u32 = match "27".trim().parse() {
    	Ok(number) => number,
    	Err(_) => DEFAULT_AGE
    };

    println!("{:?}", age);
}
