fn main() {
    let v1 = vec![4, 5, 3, 2, 1];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
}
