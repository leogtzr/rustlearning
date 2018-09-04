fn largest<T>(lst: &[T]) -> T {
	let mut largest = lst[0];
	for &number in lst.iter() {
		if number > largest {
			largest = number;
		}
	}

	largest
}

fn main() {
    
}
