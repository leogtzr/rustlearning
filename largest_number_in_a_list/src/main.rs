//fn largest(lst: &Vec<i32>) -> i32 {
fn largest(lst: &[i32]) -> i32 {
	let mut largest = lst[0];
	// for &number in lst {
	// 	if number > largest {
	// 		largest = number;
	// 	}
	// }

	for &number in lst.iter() {
		if number > largest {
			largest = number;
		}
	}
	largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is: {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
	println!("The largest number is: {}", result);    
}
