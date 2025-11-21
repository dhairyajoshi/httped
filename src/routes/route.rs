use std::collections::HashMap;

use crate::http::{
    request::Request,
    response::{Body, Response},
};

pub fn boo(request: &mut Request) -> Response {
    Response::text("boo", 200, "OK")
}

pub fn echo(request: &mut Request) -> Response {
    let request_body = match &request.body {
        Body::Json(map) => map.clone(),
        _ => HashMap::new(),
    };
    Response::json(request_body, 200, "ok")
}
