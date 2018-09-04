use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // We could also avoid the type with _, _
    let scores: HashMap<&String, &i32> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);
}
