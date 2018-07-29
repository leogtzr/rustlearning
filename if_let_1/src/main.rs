fn main() {
    let some_u8_value = Some(0u8);
    // A match that only cares about executing code when the value is Some(3)
    match some_u8_value {
        Some(3) => println!("Tres!"),
        _ => (),				// Some boilerplate code added.
    }

    // The following code fixes the previous code. Removes the boilerplate.
    if let Some(3) = Some(2u8) {
    	println!("Tres");
    } else {
    	println!("Other ... ");
    }
}
