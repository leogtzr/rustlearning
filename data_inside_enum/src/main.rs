#[derive(Debug)]
enum IpAddr {
	V4(String),				// Each variant can have different ammounts of data.
	V6(String),
	V4Individual(u8, u8, u8, u8),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    println!("{:#?}", home);

    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
    println!("{:#?}", loopback);

    let some_ip: IpAddr = IpAddr::V4Individual(127, 0, 0, 1);
    println!("{:?}", some_ip);
}
