use std::collections::HashMap;

use crate::http::{
    request::Request,
    response::{Body, Response},
};

pub fn boo(request: Request) -> Response {
    Response::text("boo", 200, "OK")
}

pub fn echo(request: Request) -> Response {
    let request_body = match request.body {
        Body::Json(map) => map,
        _ => HashMap::new(),
    };
    Response::json(request_body, 200, "ok")
}
