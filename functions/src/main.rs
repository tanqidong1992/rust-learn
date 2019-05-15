fn main() {
   let a=1;
   let b=2;
   let c=sum(a,b);
   println!("the sum of a and b is:{}",c);
}
fn sum(a:i32,b:i32)->i32{
    let c=a+b;
    c
}
