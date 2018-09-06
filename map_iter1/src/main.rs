fn main() {
    let v1 = vec![5, 4, 1];
    
    let v1_iter = v1.iter().map(|x| x + 1);

    for x in v1_iter {
        println!("x: {}", x);
    }

    println!("{:?}", v1);
}
