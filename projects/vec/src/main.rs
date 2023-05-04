struct Vec1{
    vector:Vec<i32>,
    num:i32,
}
// impl vec_init(&self::vec) ->bool {
//     vec
// }
impl Vec1{
    fn vec_display(&self)  {
        println!("vector has {} elements:",&self.num);
        for i in &self.vector {
            println!("{i}");
        }
    }
}
fn main() {
    let vector1 = Vec1{
        vector : vec![1,2,3,4,1000],
        num:1,
    };
    Vec1::vec_display(&vector1);
    println!("Hello, world!");
}
