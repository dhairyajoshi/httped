use std::{collections::HashMap, sync::Arc};

use super::handler::handle_connection;
use crate::http::{request::Request, response::Response};
use tokio::net::{TcpListener, TcpStream};

pub type HTTPHandler = Box<fn(Request) -> Response>;
pub type HTTPHandlers = HashMap<String, HTTPHandler>;
pub struct Server {
    address: String,
    handlers: HTTPHandlers,
}

impl Server {
    pub fn new(address: String) -> Server {
        Server {
            address,
            handlers: HashMap::new(),
        }
    }
    pub fn add_handler(&mut self, method: &str, path: &str, handler: fn(Request) -> Response) {
        let handler = Box::new(handler);
        self.handlers.insert(method.to_lowercase() + path, handler);
    }
    pub async fn serve(self) {
        let listener = TcpListener::bind(self.address.clone()).await.unwrap();
        let handlers = Arc::new(self.handlers);
        println!("Server started on: {}!", self.address.clone());
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            tokio::spawn(handle_connection(stream, Arc::clone(&handlers)));
        }
    }
}
