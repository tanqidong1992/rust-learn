use dh;


fn main() {

    let mut version=0;
    unsafe{
        version=dh::CLIENT_GetSDKVersion();
    }
    println!("dh sdk version:{}",version);
    let user_data:i64=0;
    unsafe {
        let initResult=dh::CLIENT_Init(Some(reconnect),user_data);
        println!("dh sdk init result:{}",initResult);
        let wait_time = 5000; // 登录请求响应超时时间设置为 5s
        let try_times = 3; // 登录时尝试建立链接 3 次
        dh::CLIENT_SetConnectTime(wait_time,try_times);

        // 此操作为可选操作
        //let mut net_param:dh::NET_PARAM=Default::default();
        //net_param.nConnectTime = 3000; // 登录时尝试建立链接的超时时间
        //dh::CLIENT_SetNetworkParam(&mut net_param);

        let mut in_param:dh::NET_IN_LOGIN_WITH_HIGHLEVEL_SECURITY=Default::default();
        
        in_param.szIP=to_c_char_array("192.168.0.199");
        in_param.nPort=37777;
        in_param.szUserName=to_c_char_array("admin");
        in_param.szPassword=to_c_char_array("admin12345");
        in_param.emSpecCap=dh::tagEM_LOGIN_SPAC_CAP_TYPE_EM_LOGIN_SPEC_CAP_TCP;
        in_param.dwSize=std::mem::size_of::<dh::NET_IN_LOGIN_WITH_HIGHLEVEL_SECURITY>() as u32;
        let mut out_param:dh::NET_OUT_LOGIN_WITH_HIGHLEVEL_SECURITY=Default::default();
        out_param.dwSize=std::mem::size_of::<dh::NET_OUT_LOGIN_WITH_HIGHLEVEL_SECURITY>() as u32;
        // 登录设备
        let login_handle = dh::CLIENT_LoginWithHighLevelSecurity(&mut in_param, &mut out_param);
        println!("login handle:{}",login_handle);
        println!("last error code:{},detail:{:?}",dh::CLIENT_GetLastError(),out_param.nError);

        //开启实时监视
        let channel_id = 0; // 预览通道号
        let emRealPlayType = dh::tagDH_RType_Realplay; // 实时监视
        std::os::raw::c_void;
        let real_handle = dh::CLIENT_RealPlayEx(login_handle, channel_id, & std::ptr::null(),emRealPlayType);
        println!("real handle:{}",real_handle);
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