enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String),
}
fn main() {
   let mut addr=IpAddr::V4(1,1,1,1);
   test(&addr);
   addr=IpAddr::V6(String::from("::1"));
   test(&addr);
}
fn test(addr:&IpAddr){
   match addr{
       IpAddr::V4(a,b,c,d) => println!("ipv4:{},{},{},{}",a,b,c,d),
       IpAddr::V6(s) => println!("ipv6:{}",s),
   };
    if let IpAddr::V4(a,b,c,d) = addr{
        println!("ipv4:{},{},{},{}",a,b,c,d);
    }


}
