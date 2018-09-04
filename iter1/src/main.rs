fn main() {
    let v1 = vec![1, 5, 7, 3, 567];
    
    let iter = v1.iter();
    
    for v in iter {
        println!("{}", v);
    }
}
