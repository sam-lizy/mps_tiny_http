use std::{net::TcpStream, io::Write, collections::HashMap};
use crate::common::{StatusCode};
pub struct Response{
    header:HashMap<String,String>,
    status_code:StatusCode,
    content:String,
}
impl Response{
    pub fn response(&self,mut stream:TcpStream){
        let status_line = format!(
            "HTTP/1.1 {} {}",
            self.status_code.get_status_line(),
            self.status_code.to_str()
        );
        let mut header:String = String::from("");
        for i in self.header.clone().into_iter(){
            header +=  &format!("\r\n{}: {}",i.0,i.1);
        }
        let response = format!(
            "{}\r\nContent-Length: {}{}\r\n\r\n{}",
            status_line,
            self.content.len(),
            header,
            self.content
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    //todo..............
    pub fn set_contentType(&mut self){
        self.header.insert(String::from("Content-Type"), String::from("application/json;charset=utf-8"));
    }
    pub fn set_res_content(&mut self,content:&str){
        self.content += content
    }   
}
impl Default for Response{
    fn default() -> Self {
        Response { 
            header: HashMap::new(), 
            status_code: StatusCode::from(200), 
            content:String::from(""),
         }
    }
}