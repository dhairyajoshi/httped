use crate::{
    http::server::Server,
    middlewares::middleware::{auth_middleware, cheeky_middleware},
    routes::route::{boo, echo},
};

mod http;
mod middlewares;
mod routes;
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let mut server = Server::new("localhost:8000");
    // server.add_middleware(auth_middleware);
    server.add_middleware(cheeky_middleware);
    server.add_handler("get", "/boo", boo);
    server.add_handler("post", "/echo", echo);
    server.add_handler("post", "/cheeky", echo);
    server.serve().await;
}
