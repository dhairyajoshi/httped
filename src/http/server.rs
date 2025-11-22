use std::{collections::HashMap, sync::Arc};

use super::handler::handle_connection;
use crate::http::{request::Request, response::Response};
use tokio::net::TcpListener;

use std::future::Future;
use std::pin::Pin;

pub type HTTPHandler =
    Box<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync>;
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
    pub fn add_handler<F, Fut>(&mut self, method: &str, path: &str, handler: F)
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let boxed: HTTPHandler = Box::new(move |req: Request| {
            Box::pin(handler(req)) as Pin<Box<dyn Future<Output = Response> + Send>>
        });

        self.handlers
            .entry(path.to_string())
            .or_default()
            .insert(method.to_lowercase(), boxed);
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
