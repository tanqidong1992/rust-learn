fn main() {

    //variable 
    /**
    let variable=5;
    variable=7;
    println!("The variable is: {}",variable);
    */
    let mut mut_var =5;
    println!("The value of mut_variable is: {}",mut_var);
    mut_var=6;
    println!("The value of mut_variable is; {}",mut_var);

    //constants,必须注明类型
    const MAX_POINTS: u32=100_000;


    //隐藏 Shadowing
    let x=5;
    let x=String::from("haha");
    // i8,u8
    // i16,u16
    // i32,u32
    // i64,u64
    // i128 u128
    // isize usize arch
    //f32 f64
    //bool : false,true

    //tup 
    let x:(i32,f32,bool)=(2,4.5,true);
    let first=x.0;

    //array
    let array=[1,3,5,,7];

    
}
