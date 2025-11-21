use super::response::Body;
use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Body,
    pub state: HashMap<String, String>,
}
