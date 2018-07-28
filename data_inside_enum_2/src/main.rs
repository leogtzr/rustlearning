#[derive(Debug)]
enum Message {
	Quit,
	Move {
		x: i32, y: i32
	},
	Write(String),
	ChangeColor(i32, i32, i32),
}


// We can have methods inside enums ... 
impl Message {
	fn call(&self) {
		println!("Holis");
	}
}

fn main() {
    println!("{:?}", "Holaaaa");

    let m1: Message = Message::Quit;
    let m2: Message = Message::Move{
    	x: 1, y: 2,
    };
    let m3: Message = Message::Write(String::from("Ok"));
    let m4: Message = Message::ChangeColor(1, 2, 3);

    println!("{:?}", m1);
    println!("{:?}", m2);
    println!("{:?}", m3);
    println!("{:?}", m4);

    m1.call();
    m2.call();
    m3.call();
    m4.call();

}
