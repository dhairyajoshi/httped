use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::http::parsers::{parse_body, parse_headers, parse_request};

pub async fn handle_connection(stream: TcpStream) {
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
    write_half.write_all(b"Hello world!").await.unwrap();
}
