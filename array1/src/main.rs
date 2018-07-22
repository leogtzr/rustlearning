fn main() {
	// Using types:
	let a: [i32; 3] = [1, 2, 3];
	println!("{:?}", a);

	// Without types:
	let b = [1, 2, 3];
	println!("{:?}", b);

	let days = ["Lunes", "Martes", "Miercoles", "Jueves", "Viernes", "Sabado", "Domingo"];
	println!("{:?}", days);

	let first_day = days[0];
	println!("The first day is: {:?}", first_day);

}