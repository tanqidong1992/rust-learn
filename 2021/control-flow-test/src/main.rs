fn main() {
    println!("Control Flow Test!");
    let number = 3;
    if number < 5 {
        println!("condition was true");
    }else{
        println!("condition was false")
    }

    if number != 0 {
        println!("number is something other than zero")
    }

    let mut b =12;
    b = if b> 10 {1}else{0};
    b=loop {
        println!("Again!");
        b+=1;
        if b>20 {
            break b;
        }
    };

    while b != 0 {
        println!("b:{}",b);
        b -= 1;
    }

    let array=[1,2,3,4,5];
    for a in array.iter(){
        println!("the value is {}",a);
    }
    for a in (1..4){
        println!("range index:{}",a);
    }
}
