fn main() {

    // let get = Method::GET("abcd".to_string());
    // let delete: Method = Method::DELETE(100);
    // let post: Method = Method::POST;
    // let put: Method = Method::PUT;

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

struct Request {
    path: String,
    query_string: String,
    method: Method,
}

enum Method {
    GET,
    DELETE,
    // GET(String),
    // DELETE(u64),
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}