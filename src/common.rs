pub struct StatusCode(u16);
impl StatusCode{
    pub fn to_str(&self)->&'static str{
        match self.0 {
            200 => "OK",
            403 => "Forbidden",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown",
        }
    }
    pub fn get_status_line(&self)->String{
        self.0.to_string()
    }
}
impl From<i16> for StatusCode{
    fn from(value:i16) -> Self {
        StatusCode(value as u16)
    }
}
pub enum Resource {
    Path(String),
}
impl Resource{
    pub fn to_url(&self)->String{
        match self {
            Resource::Path(path)=>path.to_string()
        }
    }
}
pub enum Method {
    Get,
    Post,
    Delete,
    Patch,
    Uninitialized,
}

pub enum Version {
    V1_1,
    Uninitialized,
}

impl From<&str> for Method{
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "DELETE"=>Method::Delete,
            "POST" => Method::Post,
            "PATCH" => Method::Patch,
            _ => Method::Uninitialized,
        }
    }
}
impl From<&str> for Version{
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}