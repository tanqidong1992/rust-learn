#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[link(name="avnetsdk")]
#[link(name="dhconfigsdk")]
#[link(name="dhnetsdk")]
#[link(name="StreamConvertor")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        unsafe{
            crate::CLIENT_GetSDKVersion();
        }
    }
}
