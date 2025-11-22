use std::collections::HashMap;

use crate::http::{
    request::Request,
    response::{Body, Response},
};

pub fn boo(request: &mut Request) -> Response {
    let name = match request.query_params.get("name") {
        Some(val) => val,
        None => "default",
    };
    let age = match request.query_params.get("age") {
        Some(val) => val,
        None => "20",
    };
    let greet = "boo ".to_string() + name + ", age " + age;
    Response::text(greet.as_str(), 200, "OK")
}

pub fn echo(request: &mut Request) -> Response {
    let request_body = match &request.body {
        Body::Json(map) => map.clone(),
        _ => HashMap::new(),
    };
    Response::json(request_body, 200, "ok")
}
