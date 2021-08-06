fn main() {
    let rect=Rectangle{
        width:3,
        height:2
    };
    println!("{:#?}",rect);
    println!("area:{}",rect.area());
    let square=Rectangle::square(12);
    println!("area:{}",square.area());
    let mut r=Rectangle{
        width:1,
        height:1
    };
    r.setHeight(12);
    r.setWidth(12);
    println!("{:?}",r);
}
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle {
    fn area(&self)-> u32{
        self.height*self.width
    }
    fn setWidth(&mut self,width:u32){
        self.width=width;
    }
    fn setHeight(&mut self,height:u32){
        self.height=height;
    }
    fn square(size:u32)->Rectangle{
        Rectangle{
            width:size,
            height:size
        }
    }
}
