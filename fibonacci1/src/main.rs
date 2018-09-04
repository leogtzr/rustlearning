fn fibo(n: u64) -> u64 {

	let mut i: u64 = 0;
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut c: u64 = 0;

	while i < n {
		c = a;
		a = b + a;
		b = c;
		i += 1;
	}

	return c;

}

fn main() {
    println!("{:?}", fibo(1));
    println!("{:?}", fibo(2));
    println!("{:?}", fibo(3));
    println!("{:?}", fibo(4));
    println!("{:?}", fibo(5));
    println!("{:?}", fibo(6));
}
