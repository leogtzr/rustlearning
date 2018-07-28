#[derive(Debug)]
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn build_user(email: String, username: String) -> User {
	User {
		username: username,
		email: email,
		sign_in_count: 1,
		active: false,
	}
}

fn main() {
	let user: User = User {
		username: String::from("Leo"),
		email: String::from("leogutierrezramirez@gmail.com"),
		sign_in_count: 78u64,
		active: true,
	};
	println!("{:?}", user);
	println!("{:?}", user.email);

	let mut mary = build_user(String::from("mary@gmail.com"), String::from("leogtzr"));
	mary.active = true;
	println!("{:?}", mary);

}
