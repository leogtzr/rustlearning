struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {

    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }

}

fn main() {
    let mut c = Cacher::new(|x|x);
    let x = c.value(45);
    println!("x: {}", x);

    let y = c.value(56);
    println!("y: {}", y);

    let mut cacher = Cacher::new(|vx| 1 + vx);
    println!("cache value: {}", cacher.value(32));
}
