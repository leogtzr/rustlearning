use std::io;

fn to_celcius(fah: f64) -> f64 {
	return (fah - 32.0) * (5.0 / 9.0);
}

fn main() {
   // print!("Fah: ");
   // let mut fah = String::new();
   // io::stdin().read_line(&mut fah).expect("Failed to read line");

   loop {
   		println!("Input the fah: ");
   		let mut fah = String::new();
   		io::stdin().read_line(&mut fah).expect("Failed to read line");

   		let fah: f64 = match fah.trim().parse() {
   			Ok(num) => num,
   			Err(_) => {
   				println!("Enter a valid number.");
   				continue;
   			}
   		};

   		 // println!("You entered: {}", fah);
   		 println!("{} -> {}", fah, to_celcius(fah));
   }

}
