fn main() {
    let s = String::new();
    println!("{:?}", s);

    let x: String = "Hola".to_string();
    println!("{:?}", x);

    let data = "initial contents";
    let s = data.to_string();

    println!("{:?}", s);
}
