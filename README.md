# mps_tiny_http
简单的，待完善的http框架


```Rust
{
    let server = Serve::new("127.0.0.1:7878");
    server.connect();
    for mut res in server.incoming_requests(){
        println!("{}",res.method());
        println!("{}",res.url());
        //响应请求
        res.response("6666");
    }
}
```
