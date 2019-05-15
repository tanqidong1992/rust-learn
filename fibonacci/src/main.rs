use std::io;

fn main() {
    println!("Fibonacci Sequence Generator!");
    println!("Please input n:");
    let mut n=String::new();
    io::stdin().read_line(&mut n)
        .expect("Fail to read line");
    let n:i32=match n.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("the input is not a number");
            -1
         },
    };

    let f=fibonacci(n);
    println!("the last number of fibonacci sequence is:{}",f);
}
fn fibonacci(n:i32)->i32{
   
    if n==1 {
        1
    }else if n==2{
        1
    }else{
        fibonacci(n-1)+fibonacci(n-2)
    }


}
