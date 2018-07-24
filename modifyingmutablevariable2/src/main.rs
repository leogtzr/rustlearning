fn main() {
    let mut s = String::from("Hello");
    s.push_str(" World!");
    println!("{}", s);

    {
    	let r1 = &mut s;
    	r1.push_str(" Alv!");
    }

    println!("{}", s);			// Hello World! Alv!
}
