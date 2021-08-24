fn main() {
    let str=String::from("Hello World!");

    /**
    takeOwnership(str);
    let c=str.clone();
    */
    //reference
    show(&str);
    let c=str.clone();
    show(&c);
    let mut mut_str=String::from("Hello");
    //mutable reference
    change(& mut mut_str);
    show(&mut_str);
}
fn change(str:&mut String){
    str.push_str(",HEIHIE");
}
fn show(str:&String){
    println!("{}",str);
}
fn take_ownership(str:String){
    println!("{}",str);
}
