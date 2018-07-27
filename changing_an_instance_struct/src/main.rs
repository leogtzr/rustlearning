#[derive(Debug)]
struct SimpleUser {
	username: String,
	active: bool
}

fn main() {
    let mut user: SimpleUser = SimpleUser {
    	username: String::from("leogtzr"),
    	active: true
    };

    println!("{:?}", user);
    user.active = false;

    println!("{:?}", user);

}
