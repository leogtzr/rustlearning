#[derive(Debug)]
enum SpreadSheetCell {
	Int(i32), 
	Float(f64),
	Text(String),
}

fn main() {
    let row = vec![
    	SpreadSheetCell::Int(3),
    	SpreadSheetCell::Text(String::from("Okkkkk")),
    	SpreadSheetCell::Float(56f64),
    ];

    println!("{:?}", row);
}
