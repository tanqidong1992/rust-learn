enum IpAddrKind{
    V4,
    V6,
}
enum IpAddr1{
    V4(String),
    V6(String),
}
enum IpAddr2{
    V4(u8,u8,u8,u8),
    V6(String),
}
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}
fn main() {
    println!("Hello, world!");
    let four=IpAddrKind::V4;
    let six=IpAddrKind::V6;
    let home=IpAddr{
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };
    let loopback=IpAddr{
        kind:IpAddrKind::V6,
        address: String::from("::1"),
    };
    let test=IpAddr2::V4(1,2,3,4);
    let x=if let IpAddr2::V4(a,b,c,d)=test{
        (a,b,c,d)
    };
    println!("{}",x);

}
fn route(ip_type: IpAddrKind){

}
