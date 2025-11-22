use std::collections::HashMap;
#[derive(Clone)]
pub enum Body {
    Text(String),
    Json(HashMap<String, String>),
    None,
}
pub struct Response {
    pub headers: HashMap<String, String>,
    pub body: String,
    pub status: i32,
    pub status_text: String,
}

impl Response {
    pub fn text(text: &str, status: i32, status_text: &str) -> Response {
        let mut headers = HashMap::from([("content-type".to_string(), "text/plain".to_string())]);
        headers.insert("content-length".to_string(), text.len().to_string());
        Response {
            headers,
            body: text.to_string(),
            status,
            status_text: status_text.to_string(),
        }
    }
    pub fn json(data: HashMap<String, String>, status: i32, status_text: &str) -> Response {
        let mut headers = HashMap::from([("content-type".to_string(), "text/plain".to_string())]);
        let json_body = serde_json::to_string(&data).unwrap();
        headers.insert("content-length".to_string(), json_body.len().to_string());
        Response {
            headers,
            body: json_body,
            status,
            status_text: status_text.to_string(),
        }
    }
}
