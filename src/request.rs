use crate::common::{Method, Resource, Version};
use crate::response::Response;
use std::{collections::HashMap, io::Read, net::TcpStream};
pub struct Request {
    method: Method,
    version: Version,
    headers: HashMap<String, String>,
    resource: Resource,
    body: String,
    stream: TcpStream,
    response: Response,
}

impl Request {
    pub fn new(mut stream: TcpStream) -> Result<Self,()> {
        let mut method: Method = Method::Uninitialized;
        let mut resource: Resource = Resource::Path("".to_string());
        let mut version: Version = Version::V1_1;
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut body: String = "".to_owned();
        let mut buffer = [0; 10240];

        stream.read(&mut buffer).unwrap();
        let req = String::from_utf8_lossy(&buffer[..]).to_string();
        for line in req.lines() {
            if line.contains("HTTP") {
                let (_method, _resource, _version) = process_req_line(line);
                method = _method;
                resource = _resource;
                version = _version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                headers.insert(key, value);
            } else {
                body = line.to_owned();
            }
        }

        Ok(Self {
            method,
            version,
            headers,
            resource,
            body,
            stream,
            response: Response::default(),
        })
    }
    pub fn url(&self) -> String {
        self.resource.to_url()
    }
    pub fn method(&self) -> String {
        match self.method {
            Method::Get => String::from("GET"),
            Method::Delete => String::from("DELETE"),
            Method::Patch => String::from("PATCH"),
            Method::Post => String::from("POST"),
            Method::Uninitialized => String::from("TODO"),
        }
    }
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }
    pub fn response(&mut self,content:&str) {
        self.response.set_res_content(content);
        self.response.response(self.stream.try_clone().unwrap())
    }
}
fn process_req_line(req: &str) -> (Method, Resource, Version) {
    let mut words = req.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}
fn process_header_line(req: &str) -> (String, String) {
    let mut header_items = req.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }
    (key, value)
}
