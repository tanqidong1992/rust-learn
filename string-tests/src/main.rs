fn main() {
    println!("Hello, world!");
    let mut s=String::new();
    let data="initial contents";
    let s=data.to_string();
    let s=String::from("initial contents");
    //字符串是UTF-8编码
    let s=String::from("中国");
    println!("Hello {}",s);

    let mut s1=String::from("Hello");
    s1.push_str(&s); //slice 
    s1.push('1');
    println!("has ownership:{}",s1);

    let s1=String::from("Hello");
    let s2=String::from("World");
    //let s3=s1+&s2;  // take s1 ownership
   // println!("{},{},{}",s1,s2,s3);
    let s=format!("{}-{}-{}",s1,s2,s2); //  take no ownership
    println!("{},{},{}",s1,s2,s);

    for c in s.chars(){
        println!("{}",c);
    }
    for b in s.bytes(){
        println!("{}",b);
    }
}
