use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Config {
	query: String,
	filename: String,
}

fn parse_config(args: &[String]) -> Config {
	let query = args[1].clone();
	let filename = args[2].clone();
	Config {query, filename}
}

impl Config {
	fn new(args: &[String]) -> Config {
		let query = args[1].clone();
		let filename = args[2].clone();
		Config {query, filename}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    // println!("{:?} - [{}] - [{}]", args, query, filename);

    // let mut f = File::open(filename).expect("file not found");
    // let mut contents = String::new();
    // f.read_to_string(&mut contents).expect("something went wrong reading the file");

    // println!("Contents: \n{}", contents);

    //let (query, filename) = parse_config(&args);
    //let config = parse_config(&args);
    let config = Config::new(&args);
    println!("Searching for: {:?}", config.query);
    println!("In file: {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found ... ");
}
