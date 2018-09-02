struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

fn main() {
    let calc = |x: u32| x;
    let cacher = Cacher{calculation: calc, value: Some(3)};
    // let y = cacher::calculation(34);
}
