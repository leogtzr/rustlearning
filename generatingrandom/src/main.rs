use rand::distributions::{IndependentSample, Range};

fn main() {
    let between = Range::new(1, 100);
    let mut rng = rand::thread_rng();

    let total = 10;

    for _ in 0..total {
    	let a = between.ind_sample(&mut rng);
    	println!("Val: {}", a);
    }
}
