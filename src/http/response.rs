use std::collections::HashMap;

pub enum Body {
    Text(String),
    Json(HashMap<String, String>),
    None,
}
pub struct Response {
    headers: HashMap<String, String>,
    body: Body,
    status: i32,
}
