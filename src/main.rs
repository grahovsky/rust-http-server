fn main() {
    
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();

}

pub struct Server {

    addr: String,

}

    

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }
    
    pub fn run(&self) {
       println!("Listening on {}", self.addr);
    }
}
