use crate::{
    http::server::Server,
    routes::route::{boo, echo},
};

mod http;
mod routes;
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let mut server = Server::new("localhost:8000");
    server.add_handler("get", "/boo", boo);
    server.add_handler("post", "/echo", echo);
    server.serve().await;
}
