
#[derive(Debug)]
struct Rectangle{
    width:i32,
    height:i32,
}
impl Rectangle {
    fn area(&self)->i32{
        self.width*self.height
    }
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width>other.width && self.height>other.height
    }
    fn square(size:i32) ->Rectangle{
        Rectangle{
            width:size,
            height:size,
        }
    }
}

struct Rect(u32,u32);

fn main() {
    println!("Hello, world!");
    let width=10;
    let height=13;
    let area=area(width, height);
    println!("The area of the rectangle is {} square pixels.",area);
    let rect=Rectangle{
        width:10,
        height:10,
    };
    let area=areaExt(&rect);
    println!("The area of the rectangle is {} square pixels.",area);
    
    println!("rect:? is {:?}",rect);
    println!("rect:#? is {:#?}",rect);

    println!("The area of the rect {:?} is {} ",rect,rect.area());

    let rect1=Rect(13,13);
    let area=areaExtTuple(&rect1);
    println!("The area of the rectangle is {} square pixels.",area);



}

fn area(width:i32,height:i32)->i32{
    width*height     
}

fn areaExt(rect:&Rectangle)->i32{
    rect.width*rect.height     
}

fn areaExtTuple(rect:&Rect)->u32{
    rect.0*rect.1     
}
