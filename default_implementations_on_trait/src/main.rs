trait ToString {
	fn to_string(&self) -> String {
		String::from("(alv)")
	}
}

#[derive(Debug)]
struct Tamal {
	peso: f64,
}

impl ToString for Tamal {
	// Overriding the default implementation ... 
	fn to_string(&self) -> String {
		String::from("alrpvw")
	}
}

fn main() {
    let t: Tamal = Tamal{peso: 45.6};
    println!("{:?}", t);
    println!("{}", t.to_string());
}
