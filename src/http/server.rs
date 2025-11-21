use std::{collections::HashMap, sync::Arc};

use super::handler::handle_connection;
use crate::http::{request::Request, response::Response};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    address: String,
    handlers: HashMap<String, fn(Request) -> Response>,
}

impl Server {
    pub fn new(address: String) -> Server {
        Server {
            address,
            handlers: HashMap::new(),
        }
    }
    fn add_handler(&mut self, method: String, path: String, handler: fn(Request) -> Response) {
        self.handlers.insert(method + path.as_str(), handler);
    }
    pub async fn serve(self) {
        let listener = TcpListener::bind(self.address.clone()).await.unwrap();
        println!("Server started on: {}!", self.address.clone());
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            tokio::spawn(handle_connection(stream));
        }
    }
}
