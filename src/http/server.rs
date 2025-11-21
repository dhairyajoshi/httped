use std::{collections::HashMap, sync::Arc};

use super::handler::handle_connection;
use crate::http::{request::Request, response::Response};
use tokio::net::TcpListener;

pub type HTTPHandler = fn(&mut Request) -> Response;
pub enum MiddlewareResponse {
    Next(),
    Response(Response),
}
pub type Middleware = fn(&mut Request) -> MiddlewareResponse;
pub type HTTPHandlers = HashMap<String, HashMap<String, HTTPHandler>>;
pub struct Server {
    address: String,
    handlers: HTTPHandlers,
    middlewares: Vec<Middleware>,
}

impl Server {
    pub fn new(address: &str) -> Server {
        Server {
            address: address.to_string(),
            handlers: HashMap::new(),
            middlewares: Vec::new(),
        }
    }
    pub fn add_handler(&mut self, method: &str, path: &str, handler: fn(&mut Request) -> Response) {
        match self.handlers.get(path) {
            Some(dict) => {
                let mut updated = dict.clone();
                updated.insert(method.to_lowercase().to_string(), handler);
                self.handlers.insert(path.to_string(), updated);
            }
            None => {
                let dict: HashMap<String, HTTPHandler> =
                    HashMap::from([(method.to_lowercase().to_string(), handler)]);
                self.handlers.insert(path.to_string(), dict);
            }
        };
    }
    pub fn add_middleware(&mut self, middleware: Middleware) {
        self.middlewares.push(middleware);
    }
    pub async fn serve(self) {
        let listener = TcpListener::bind(self.address.clone()).await.unwrap();
        let handlers = Arc::new(self.handlers);
        let middlewares = Arc::new(self.middlewares);
        println!("Server started on: {}!", self.address.clone());
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            tokio::spawn(handle_connection(
                stream,
                Arc::clone(&middlewares),
                Arc::clone(&handlers),
            ));
        }
    }
}
