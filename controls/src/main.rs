fn main() {
    let a=7;
    let b=8;
    if a>b {
        println!(" a greater then b");
    }else{
        println!("a less then b");
    }

    let c= if a>b {
        let d=1;
        3
    }else{
        4
    };
    /**
    if a>b {
        let d=1;
        3
    }else{
        4
    }
    */
    println!("the c value:{}",c);

    //empty loop
    /**
    loop{
        println!("loop again!")
    }
    */
    let mut i=0;
    let x=loop{
        
        if i>10 {
            break i;
        }
        i+=1;
    };
    println!("loop return value x:{}",x);

    let items=[1,2,3,4,5];
    for item in items.iter(){
        println!("{}",item);
    };
    
}
