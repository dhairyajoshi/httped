use crate::http::server::Server;

mod http;
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let server = Server::new("localhost:8000".to_string());
    server.serve().await;
}
