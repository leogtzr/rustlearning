fn main() {
    let v = vec![100, 32, 56];
    for i in &v {
    	println!("{:?}", i);
    }

    /*
    	Making changes to all the elements.
    */
    let mut v2 = vec![100, 32, 45];
    for e in &mut v2 {
    	*e += 50;
    }

    println!("{:?}", v2);
}
