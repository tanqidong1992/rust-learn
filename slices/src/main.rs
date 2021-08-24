fn main() {
   let s=String::from("Hello world");
   //include the end index
   let hello=&s[0..=5];
   //exclude the end index
   let world=&s[6..10];
   //from 0 to 10
   let a=&s[..10];
   //from 1 to 11
   let b=&s[1..];
   println!("|{}|",hello);
   println!("|{}|",world);
   let fw=first_world(&s);
   //s.clear();
   println!("the first world of:{} is {}",s,fw);
   //array slice
   let a=[1,2,3,4,5];
   let slice=&a[1..3];//the type of slice is &[i32];
}
fn first_world(line:&str) -> &str{
    let bytes=line.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item ==b' '{
            return &line[..i];
        }
    }
    &line[..]
}
