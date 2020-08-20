use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("120.0.0.1:8080".to_string());
    server.run()
}

