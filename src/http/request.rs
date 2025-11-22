use super::response::Body;
use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub query_params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Body,
    pub state: HashMap<String, String>,
}
