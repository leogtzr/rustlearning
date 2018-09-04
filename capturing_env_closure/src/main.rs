fn main() {

    let x = 3;
    let equal_to_x = |y| y == x;
    let y = 3;
    
    if equal_to_x(y) {
        println!("They are equal :)");
    } else {
        println!("They are NOT equal, {}", x);
    }

}
