extern crate minigrep;

use std::env;
use std::error::Error;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
    	println!("Problem parsing arguments: {}", err);
		process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
    	println!("Application error: {}", e);
    	process::exit(1);
    };
}

mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}