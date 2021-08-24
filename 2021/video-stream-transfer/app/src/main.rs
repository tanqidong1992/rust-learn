use dh;
fn main() {
    println!("Hello, world!");
    let mut version=0;
    unsafe{
        version=dh::CLIENT_GetSDKVersion();
    }
    println!("dh sdk version:{}",version);
}
