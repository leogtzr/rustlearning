use std::collections::HashMap;

fn main() {

    let field_name: String = String::from("Favorite color");
    let field_value: String = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", map);

    // field_name: String, String does not implement Copy trait so it can't be copied.
    // print!("{:?}", field_name);
    // print!("{:?}", field_value);
}
