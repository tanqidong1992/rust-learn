struct User{
    name:String,
    address:String,
    age:u32,
}
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}
impl Rectangle{
    //to change self using: &mut self
    fn area(&self,s:&str)->u32{
        println!("invoke with str:{}",s);
        self.width*self.height
    }
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width>other.width && self.height>other.height
    }
    fn square(size:u32)->Rectangle{
        Rectangle{
            width:size,
            height:size
        }
    }
}
fn main() {
    println!("Hello, world!");
    let user=User{
        name:String::from("tqd"),
        address:String::from("Heihei"),
        age:1
    };
    //consider changing this(user) to be mutable,
    //user.name=String::from("tqd1");
    let width=30;
    let height=50;
    println!("the area of the rectangle is {} square pixels",area(width,height));
    let rect=(30,50);
    println!("the area of the rectangle is {} square pixels",area1(rect));
    let rect1=Rectangle{
        width:30,
        height:50
    };
    println!("the area of the rectangle is {} square pixels",area2(&rect1));
    println!("the rectangle is:{:?}",rect1);
    let s=String::from("tqd");
    println!("the area of the rectangle is {} square pixels",rect1.area(&s));

    let rect2=Rectangle{
        width:50,
        height:60
    };
    let rect3=Rectangle{
        width:10,
        height:20
    };
    println!("rect1 can hold rect2 :{}",rect1.can_hold(&rect2));
    println!("rect1 can hold rect3 :{}",rect1.can_hold(&rect3));
    let square=Rectangle::square(12);
    println!("the square is:{:?}",square);
}
fn area2(rect:&Rectangle)->u32{
    rect.width*rect.height
}
fn area1(dimensions:(u32,u32))->u32{
    dimensions.0*dimensions.1
}
fn area(width:u32,height:u32)->u32{
    width*height
}
