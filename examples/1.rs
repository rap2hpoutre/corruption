extern crate env_logger;
extern crate futures;
extern crate corruption;
extern crate time;

use std::net::SocketAddr;
use std::env;

use futures::*;
use corruption::{Server, Response, Request, Router};

fn main() {


    


    env_logger::init().unwrap();
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();
    Server::new(&addr).serve(|r: Request| {


        let mut a = Router::new();
        a.get("pouet", |r: Request| println!("a{}", r.path()));
        a.get("pouet2", |r: Request| println!("a{}", r.path()));
        a.action(0, r);



        // assert_eq!(r.path(), "/plaintext");
        let mut r = Response::new();
        r.header("Content-Type", "text/plain; charset=UTF-8")
         .body("Hello, World!");
        finished::<_, std::io::Error>(r)
    }).unwrap()
}