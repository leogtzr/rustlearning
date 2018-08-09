#[derive(Debug)]
struct Point<T, U> {
	x: T,
	y: U,
}

fn main() {
    let x: Point<i32, f32> = Point{x: 1, y: 4.03};
    println!("x: {:?}", x);

    let y = Point{x: "Hola", y: "Mundo"};
    println!("{:?}", y);

    let float = Point{x: 1.23, y: 456.34};
    println!("{:?}", float);
}