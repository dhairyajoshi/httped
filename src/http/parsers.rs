use std::collections::HashMap;

use tokio::{
    io::{AsyncReadExt, BufReader},
    net::tcp::OwnedReadHalf,
};

use crate::http::{
    request::Request,
    response::{Body, Response},
};
pub fn parse_headers(headers: Vec<String>) -> HashMap<String, String> {
    let mut header_map = HashMap::new();
    for header in headers {
        let parts: Vec<String> = header
            .split(":")
            .map(String::from)
            .map(|e| e.trim().to_lowercase().to_string())
            .collect();
        header_map.insert(parts[0].clone(), parts[1].clone());
    }
    header_map
}
pub async fn parse_body(
    headers_map: &HashMap<String, String>,
    reader: &mut BufReader<OwnedReadHalf>,
) -> Body {
    let content_length: usize = match headers_map.get("content-length") {
        Some(val) => val.parse().unwrap(),
        None => 0,
    };
    let mut bytes = vec![0; content_length];
    let mut body: Body = Body::None;
    if content_length > 0 {
        reader.read_exact(&mut bytes).await.unwrap();
        body = match headers_map.get("content-type") {
            Some(content_type) => match content_type.as_str() {
                "application/json" => {
                    let body_json: HashMap<String, String> =
                        serde_json::from_str(String::from_utf8(bytes).unwrap().as_str()).unwrap();
                    Body::Json(body_json)
                }
                "text/plain" => Body::Text(String::from_utf8(bytes).unwrap()),
                _ => Body::None,
            },
            None => Body::None,
        }
    }
    body
}
pub fn parse_request(
    request_line: String,
    headers: &HashMap<String, String>,
    body: Body,
) -> Request {
    let parts: Vec<String> = request_line.split_whitespace().map(String::from).collect();

    Request {
        method: parts[0].clone(),
        path: parts[1].clone(),
        headers: headers.clone(),
        body,
        state: HashMap::new(),
    }
}

pub fn prepare_response(response: Response) -> String {
    let mut buffer = String::new();

    buffer.push_str("HTTP/1.1 ");
    buffer.push_str(&response.status.to_string());
    buffer.push_str(" ");
    buffer.push_str(&response.status_text);
    buffer.push_str("\r\n");

    for (k, v) in &response.headers {
        buffer.push_str(k);
        buffer.push_str(": ");
        buffer.push_str(v);
        buffer.push_str("\r\n");
    }
    buffer.push_str("Connection: close\r\n");
    buffer.push_str("\r\n");
    buffer.push_str(response.body.as_str());

    buffer
}
