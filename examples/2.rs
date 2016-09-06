extern crate corruption;
extern crate hyper;
extern crate rustc_serialize;

use hyper::server::Request;
use corruption::Corruption;
use corruption::response::Response;


fn main() {
    // Start Corruption
    let mut corruption = Corruption::new();

    // Declare routes
    corruption
        .get("/json", json_repsonse )
        .get("/html", html_repsonse );

    // Serve it to the world on 127.0.0.1:8080
    corruption.serve();
}

fn html_repsonse(_: &Request) -> corruption::response::Response {
    Response::html_str("hello")
}

#[derive(RustcEncodable)]
struct ExampleStruct {
    a: u8,
    b: String,
    c: Vec<u8>
}

fn json_repsonse(_: &Request) -> corruption::response::Response {
    Response::json(&ExampleStruct {
        a: 54,
        b: "hello".to_string(),
        c: vec![2,3,4,5],
    })
}