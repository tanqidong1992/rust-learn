fn main() {
    let a:Vec<i32>=Vec::new();
    let mut v=vec![1,2,3];
    v.push(4);
    let third: &i32=&v[2];
    println!("the third element is:{}",third);
    match v.get(2) {
        Some(third) => println!("the third element is:{}",third),
        None => println!("Not equal"),
    }
    // panick
    //let e1=&v[4];
    //Option
    let e=v.get(4);
    match e{
        Some(a) => println!("the 4 element is:{}",a),
        None=>println!("the 4 element is:None"),
    }
    //imutable reference
    for i in &v{
        println!("{}",i);
    }
    //mutable reference
    for i in &mut v{
        *i+=50;
    }
     for i in &v{
        println!("{}",i);
    }
    let row=vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(2.0),
        SpreadsheetCell::Text(String::from("Heibei")),
    ];
    for c in &row{
        if let SpreadsheetCell::Int(a)=c{
           println!("the Int value:{}",a); 
        }
    }
    for c in &row{
        match c {
            SpreadsheetCell::Int(a)=>println!("int:{}",a),
            SpreadsheetCell::Float(a)=>println!("float:{}",a),
            SpreadsheetCell::Text(a)=>println!("text:{}",a),
        }
    }
}
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}
