use std::{collections::HashMap, sync::Arc};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::http::{
    parsers::{parse_body, parse_headers, parse_request, prepare_response},
    request::Request,
    response::Response,
    server::{HTTPHandler, HTTPHandlers},
};

fn handle_request(request: Request, handlers: Arc<HTTPHandlers>) -> Response {
    let key = request.method.to_lowercase() + request.path.as_str();
    match handlers.get(&key) {
        Some(handler) => handler(request),
        None => Response::text("404 Not found".to_string(), 404, "NOT_FOUND".to_string()),
    }
}

pub async fn handle_connection(stream: TcpStream, handlers: Arc<HTTPHandlers>) {
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);
    let mut ln = 0;
    let mut request_line = String::from("");
    let mut headers = Vec::new();
    loop {
        let mut input = String::new();
        reader.read_line(&mut input).await.unwrap();
        if input == "\r\n" {
            break;
        }
        if ln == 0 {
            request_line = input.trim().to_string();
            ln += 1;
        } else {
            headers.push(input.trim().to_string());
        }
    }
    let headers_map = parse_headers(headers);
    let body = parse_body(&headers_map, &mut reader).await;
    let request = parse_request(request_line, &headers_map, body);
    let response = handle_request(request, handlers);
    let server_response = prepare_response(response);
    write_half
        .write_all(server_response.as_bytes())
        .await
        .unwrap();
}
