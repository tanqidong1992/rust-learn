struct User{
    name:String,
}
impl User{
    fn new(name:String)->User{
        User{
            name
        }
    }
    fn nameValue(&self)->String{
        self.name
    }
}
fn main() {
    let user=User::new(String::from("tqd"));
    println!("Hello, {}",user.nameValue());
}
