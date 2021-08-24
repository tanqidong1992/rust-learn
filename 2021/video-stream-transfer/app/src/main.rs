use dh;
fn main() {
    println!("Hello, world!");
    let mut version=0;
    unsafe{
        version=dh::CLIENT_GetSDKVersion();
    }
    println!("dh sdk version:{}",version);
    let userData:i64=0;
    unsafe {
        let initResult=dh::CLIENT_Init(Some(reconnect),userData);
        println!("dh sdk init result:{}",initResult);
        let nWaitTime = 5000; // 登录请求响应超时时间设置为 5s
        let nTryTimes = 3; // 登录时尝试建立链接 3 次
        dh::CLIENT_SetConnectTime(nWaitTime,nTryTimes);

        // 此操作为可选操作
        //let stuNetParm = dh::NET_PARAM{};
        //stuNetParm.nConnectTime = 3000; // 登录时尝试建立链接的超时时间
        //dh::CLIENT_SetNetworkParam(stuNetParm);

        let mut stInparam:dh::NET_IN_LOGIN_WITH_HIGHLEVEL_SECURITY=Default::default();
        
        stInparam.szIP=to_c_char_array("192.168.0.199");
        stInparam.nPort=8888;
        stInparam.szUserName=to_c_char_array("admin");
        stInparam.szPassword=to_c_char_array("admin12345");
        stInparam.emSpecCap=dh::tagEM_LOGIN_SPAC_CAP_TYPE_EM_LOGIN_SPEC_CAP_TCP;

        let mut stOutParam:dh::NET_OUT_LOGIN_WITH_HIGHLEVEL_SECURITY=Default::default();
        /**
            dwSize:0,
            stuDeviceInfo:Default::default(),
            nError:0,
            byReserved:[0l,132]
        };
        */
        // 登录设备
        let g_lLoginHandle = dh::CLIENT_LoginWithHighLevelSecurity(&mut stInparam, &mut stOutParam);
        println!("login handle:{}",g_lLoginHandle);

    }


}
fn to_c_char_array(s:&str)-> [c_char;64]{
    let i8_collection:Vec<c_char>=s.as_bytes().iter().map(|x|*x as i8).collect();
    let mut u8_array:[c_char;64]=[0;64];
    for i in 0..i8_collection.len(){
        u8_array[i]=i8_collection.get(i).unwrap().clone();
    }
    u8_array
}
use std::os::raw::{
    c_long,
    c_char,
    c_int
};

unsafe extern "C" fn reconnect(lLoginID: c_long, pchDVRIP: *mut c_char, nDVRPort: c_int, dwUser: c_long){

}