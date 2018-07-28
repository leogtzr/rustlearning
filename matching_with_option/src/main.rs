fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}

fn main() {
    let x: Option<i32> = Some(34);
    //let x: Option<i32> = None;			Also valid.
    match plus_one(x) {
    	None => {
    		println!("Nothing to do here ... ");
    	},
    	Some(v) => {
    		println!("Value: {}", v);
    	},
    }
}
