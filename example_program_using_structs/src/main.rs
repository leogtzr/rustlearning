#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // let rect1: (u32, u32) = (30, 50);

    let r1: Rectangle = Rectangle{width: 30u32, height: 50u32};

    println!("{:#?}", r1);

    //println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    // println!("The area of the rectangle is {} square pixels.", area(rect1));
	println!("The area of the rectangle is {} square pixels.", area(&r1));
}

// fn area(width: u32, height: u32) -> u32 {
// 	width * height
// }

fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}