use request::Request;
use response::Response;
use server::Server;
use router::Router;
use std::net::SocketAddr;
use futures::*;
use std;

pub struct Corruption {
    router: Router,
}

impl Corruption {

    pub fn new() -> Corruption {
        Corruption {
            router: Router::new()
        }
    }

    pub fn get<T: 'static + Fn(Request)>(&mut self, uri: &str, handler: T) {
        self.router.get(uri, handler)
    }

    pub fn serve(&mut self, addr: &str) {
        let socket_addr = addr.to_string().parse::<SocketAddr>().unwrap();
        Server::new(&socket_addr).serve(|r: Request| {
            println!("{:?}", r.path());
            let mut r = Response::new();
            r.header("Content-Type", "text/plain; charset=UTF-8")
                .body("Hello, World!");
            finished::<_, std::io::Error>(r)
        }).unwrap()
    }
}