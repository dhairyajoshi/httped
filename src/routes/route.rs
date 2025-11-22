use std::collections::HashMap;

use crate::http::{
    request::Request,
    response::{Body, Response},
};

pub async fn boo(request: Request) -> Response {
    let name = request
        .query_params
        .get("name")
        .cloned()
        .unwrap_or("default".to_string());

    let age = request
        .query_params
        .get("age")
        .cloned()
        .unwrap_or("20".to_string());

    let greet = format!("boo {}, age {}", name, age);

    Response::text(greet.as_str(), 200, "OK")
}

pub async fn echo(request: Request) -> Response {
    let request_body = match &request.body {
        Body::Json(map) => map.clone(),
        _ => HashMap::new(),
    };

    Response::json(request_body, 200, "ok")
}
