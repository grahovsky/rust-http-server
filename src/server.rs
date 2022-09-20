use std::{net::TcpListener, io::Read};

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

        let listener = TcpListener::bind(&self.addr).unwrap();

        'outer: loop {

            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer);
                },
                Err(e) => println!("Failed to establish a connection {}", e),
            };

        }
        
    }
}
