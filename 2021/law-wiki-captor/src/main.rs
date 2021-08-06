use hyper::client::Client;
use hyper::{Request, Method, Body, Uri};
use std::any::Any;

const BASE_URL:&str="https://law.wkinfo.com.cn";
const LOGIN_URL:&str="http://law.wkinfo.com.cn/csi/account/validate/ex";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let client=Client::new();
    println!("{}",LOGIN_URL);
    let uri:Uri=LOGIN_URL.parse()?;
    let body=Body::from("
    {
  \"username\": \"qidongtan@qq.com\",
  \"password\": \"903843602Aa\"
}
    ");
    let req=Request::builder()
        .method(Method::POST)
        .header("Origin","https://law.wkinfo.com.cn/")
        .header("Referer","https://law.wkinfo.com.cn/")
        .header("X-Tingyun-Id","tN6Win9ZeY4;r=76361289")
        .header("Sec-Fetch-Mode","cors")
        .header("Sec-Fetch-Site","same-origin")
        .uri(uri)
        .body(body)?;
    let resp=client.request(req).await?;
    println!("resp status code:{}",resp.status());
    if resp.status().is_redirection(){
        //resp.headers().get()
    }
    //println!("{}",resp);
    Ok(())
}
