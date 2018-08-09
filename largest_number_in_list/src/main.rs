use std::cmp::PartialOrd;

//fn largest<T: PartialOrd + Copy>(lst: &[T]) -> T {
fn largest<T>(lst: &[T]) -> T where T: PartialOrd + Copy {
	let mut largest = lst[0];
	for &i in lst.iter() {
		if i > largest {
			largest = i;
		}
	}
	largest
}

fn main() {
    let lst = vec![5, 3, 77, 4];
    let lg = largest(&lst);
    println!("{}", lg);

    let lst_f: Vec<f64> =  vec![3.4, 67.345, 78.56];
    let lg = largest(&lst_f);

    println!("{}", lg);
}
