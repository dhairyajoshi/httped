use std::sync::Arc;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::http::server::MiddlewareResponse;
use crate::http::{
    parsers::{parse_body, parse_headers, parse_request, prepare_response},
    request::Request,
    response::Response,
    server::{HTTPHandlers, Middleware},
};

async fn handle_request(
    request: &mut Request,
    middlewares: Arc<Vec<Middleware>>,
    handlers: Arc<HTTPHandlers>,
) -> Response {
    match handlers.get(request.path.as_str()) {
        Some(methods) => match methods.get(request.method.to_lowercase().as_str()) {
            Some(handler) => {
                for middleware in middlewares.iter() {
                    match middleware(request) {
                        MiddlewareResponse::Next() => continue,
                        MiddlewareResponse::Response(res) => return res,
                    };
                }
                handler(request.clone()).await
            }
            None => Response::text("405 Method Not Allowed", 405, "METHOD_NOT_ALLOWED"),
        },
        None => Response::text("404 Not found", 404, "NOT_FOUND"),
    }
}

pub async fn handle_connection(
    stream: TcpStream,
    middlewares: Arc<Vec<Middleware>>,
    handlers: Arc<HTTPHandlers>,
) {
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
    let mut request = parse_request(request_line, &headers_map, body);
    let response = handle_request(&mut request, middlewares, handlers).await;
    let server_response = prepare_response(response);
    write_half
        .write_all(server_response.as_bytes())
        .await
        .unwrap();
}
