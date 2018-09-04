#[derive(Debug)]
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn build_username(email: String, username: String) -> User {
	/*
		Since there is a variable email and username and they match the fields
		in the struct then we can use this shorthand.
	*/
	User {
		email,
		username,
		sign_in_count: 1,
		active: true,
	}
}

fn main() {
    let user: User = 
    	build_username(String::from("leogutierrezramirez@gmail.com"), String::from("leogtzr"));
    println!("{:?}", user);
}
