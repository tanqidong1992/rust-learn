fn main() {
    let list:[i32;5]=[1,2,3,4,5];
    println!("the largest i32 is:{}",largest_i32(&list));
    let list:[char;5]=['a','b','c','d','e'];
    println!("the largest char is:{}",largest_char(&list));
    let floatPoint=Point{
        x:1.0,
        y:2.0,
    };
    let intPoint=Point{
        x:1,
        y:2,
    };
}
fn largest<T>(list:&[T])->T{
    let mut largestItem=list[0];
    for &i in list.iter(){
        if i>largestItem{
            largestItem=i;
        }
    }
    largestItem;
}
fn largest_char(list:&[char])->char{
    let mut largest=list[0];
    for &c in list.iter(){
        if c >largest {
            largest=c;
        }
    }
    largest
}
fn largest_i32(list:&[i32])->i32{
    let mut largest=list[0];
    for &i in list.iter(){
        if i>largest{
            largest=i;
        }
    }
    largest
}
struct Point<T>{
    x:T,
    y:T,
}
struct Point1<V,U>{
    x:V,
    y:U,
}
enum UserType<T>{
    Student(T),
    None,
}
impl Point<f32>{
    fn distance_from_origin(&self)->f32{
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
}