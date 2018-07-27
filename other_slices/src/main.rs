fn main() {
    let a = [2, 3, 5, 7, 11];
    let slice: &[i32] = &a[1..3];				// [3, 5]: &[i32]
    println!("{:?}", slice);
}
