#[derive(Debug)]
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn main() {
	let user: User = User {
		username: String::from("Leo"),
		email: String::from("leogutierrezramirez@gmail.com"),
		sign_in_count: 78u64,
		active: true,
	};
	println!("{:?}", user);

}
