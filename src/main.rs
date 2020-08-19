fn main() {
    let address = String::from("120.0.0.1:8080".to_string());

    let server = Server::new(address);

    server.run()
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    fn run(self) {
        println!("Listening on {}", self.addr)
    }
}
