pub fn add_two(a: i32) -> i32 {
	a + 2
}

#[cfg(test)]
mod tests {
	
	use super::*;
	
	#[test]
	fn add_two_and_two() {
		assert_eq!(4, add_two(2));
	}

	// The following test will be ignored due to the use of #[ignore]	
	#[test]
	#[ignore]
	fn add_one() {
		assert_eq!(2, add_two(0));
	}

}
