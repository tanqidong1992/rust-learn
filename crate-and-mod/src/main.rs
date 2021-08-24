mod aa; 
use aa::bb as hh1;
mod hh{

    pub fn sayHello(){
        println!("Hello, world!");
    }
}
fn main() {
    hh::sayHello();
    hh1::say();
}
