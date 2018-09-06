#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
    .filter(|s| s.size == shoe_size)
    .collect()
}

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe {size: 10, style: String::from("alv")},
        Shoe {size: 54, style: String::from("hmmm")},
        Shoe {size: 10, style: String::from("holis")}
    ];
    
    println!("{:?}", shoes); 
}
