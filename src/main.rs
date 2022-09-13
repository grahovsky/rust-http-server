use http::request::Request;
use server::Server;

mod server;

fn main() {

    // let get = Method::GET("abcd".to_string());
    // let delete: Method = Method::DELETE(100);
    // let post: Method = Method::POST;
    // let put: Method = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();

}

mod http {

    pub mod request {
        use super::method::Method;
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: Method,
        }
    }

    pub mod method {
        pub enum Method {
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
    }
}