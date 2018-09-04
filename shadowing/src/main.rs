fn main() {
    let x = 5;
    let x = x + 1;			// 6
    let x = x * 2;			// 12
    println!("{:?}", x);

    // Since we are declaring a new variable, we can use a different type:
    let x: bool = true;
    println!("{:?}", x);

    let spaces = " ";
    // a brand new variable.
    // This is very handy because we avoid the creation of a lot of temporary variabes.
    let spaces = spaces.len();

    println!("{:?}", spaces);
}