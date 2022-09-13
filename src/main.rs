use http::Method;
use http::Request;
use server::Server;

mod server;
mod http;

fn main() {

    // let get = Method::GET("abcd".to_string());
    // let delete: Method = Method::DELETE(100);
    // let post: Method = Method::POST;
    // let put: Method = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();

}