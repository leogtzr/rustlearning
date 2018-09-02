fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("holaaaaaaaa")); 
    let x = example_closure(2);
    
    println!("s: {}", s);
    println!("x: {}", x);
}
