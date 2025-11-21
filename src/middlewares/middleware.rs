use std::collections::HashMap;

use crate::http::{request::Request, response::Response, server::MiddlewareResponse};

pub fn cheeky_middleware(request: &mut Request) -> MiddlewareResponse {
    if request.path.as_str() == "/cheeky" {
        request.body = crate::http::response::Body::Json(HashMap::from([(
            "msg".to_string(),
            "cheeky endpoint".to_string(),
        )]));
        MiddlewareResponse::Next()
    } else {
        MiddlewareResponse::Next()
    }
}

pub fn auth_middleware(request: &mut Request) -> MiddlewareResponse {
    MiddlewareResponse::Response(Response::json(
        HashMap::from([("msg".to_string(), "unauthorized".to_string())]),
        401,
        "unauthorized",
    ))
}
