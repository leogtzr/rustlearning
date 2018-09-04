use std::collections::HashMap;

fn max_key_in_map(m: &HashMap<&i32, i32>) -> i32 {
	let mut max_key: &i32 = &0;
	let mut max_value: &i32 = &0;
	for (&k, v) in m {
		if v > max_value {
			max_key = k;
			max_value = v;
		}
	}
	return *max_key
}

fn main() {
	let mut nums = vec![5, 2, 3, 4, 1, 1];
	let mut sum = 0.0;

	for v in &nums {
		sum += *v as f64;
	}

	let mean: f64 = sum / (nums.len() as f64);
	println!("Mean: {:?}", mean);

	nums.sort();

	println!("Nums: {:?}", nums);

	println!("Median: {:?}", nums[ nums.len() / 2 ]);

	println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

	let mut map = HashMap::new();
	for v in &nums {
		let count = map.entry(v).or_insert(0);
    	*count += 1;
	}

	println!("{:?}", nums);
	println!("{:?}", map);

	println!("################################################################");

	let mx: i32 = max_key_in_map(&map);
	println!("Most repeated values -> {:?} ===> {:?}", 
		mx, 
		map.get(&mx).unwrap());

}
