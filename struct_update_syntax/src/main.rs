#[derive(Debug)]
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool
}

fn build_user(username: String, email: String) -> User {
	User {
		username,
		email,
		sign_in_count: 1u64,
		active: false
	}
}

fn main() {
	let username = String::from("leogtzr");
	let email = String::from("leogutierrezramirez@gmail.com");
    let user1: User = build_user(username, email);

    let user2: User = User {
    	username: String::from("something"),
    	email: String::from("something@gmail.com"),
    	.. user1						// Struct update syntax.
    };

    println!("{:?}", user1);
    println!("{:?}", user2);
}