fn main() {
   let mut v = vec![1, 2, 3];
   println!("{:?}", v);

   v.push(4);
   v.push(5);

   println!("{:?}", v);

   let mut third = &v[2];
   third = &345i32;
   println!("{:?}", third);

   println!("{:?}", v);

   let third: Option<&i32> = v.get(2);
   match third {
       Some(n) => println!("{:?}", n),
       None => println!("Empty ... "),
   }
}
