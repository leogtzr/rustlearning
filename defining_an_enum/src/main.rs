#[derive(Debug)]
enum IpAddrKind {
	V4,
	V6,
}

fn route(_ip_kind: IpAddrKind) {}				// _ip_kind to make it explicit that the variable is not used.

#[derive(Debug)]
struct IpAddr {
	ip_kind: IpAddrKind,
	address: String,
}

fn main() {
    let ip_kind: IpAddrKind = IpAddrKind::V4;
    println!("{:?}", ip_kind);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr{
    	ip_kind: IpAddrKind::V4,
    	address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
    	ip_kind: IpAddrKind::V6,
    	address: String::from("::1"),
    };
    println!("{:#?}", home);
    println!("{:#?}", loopback);
}
