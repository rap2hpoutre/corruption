extern crate hyper;
extern crate mildew;
extern crate handlebars;
extern crate rustc_serialize;
extern crate time;

use std::io::{Write};
use hyper::*;

pub mod response;

pub type Route = (method::Method, String, Box<Fn(&server::Request) -> self::response::Response + Send + Sync>);

pub struct Corruption {
    handler: MyHandler,
    addr: &'static str,
}

pub struct MyHandler {
    routes: Vec<Route>
}

impl MyHandler {
    pub fn new() -> MyHandler {
        MyHandler {
            routes: Vec::new()
        }
    }
}

impl server::Handler for MyHandler {
    fn handle(&self, req: server::Request, mut res: server::Response) {

        println!("Access: {}", req.uri);

        let routes = &self.routes;
        let mut route = None;
        for i in routes {
            if req.method == i.0 && format!("{}", req.uri) == format!("{}", &i.1) {
                route = Some(i);
                break;
            }
        }

        let corruption_response: self::response::Response = match route {
            None => { *res.status_mut() = status::StatusCode::NotFound; self::response::Response::html("404.html")},
            Some(r) => (r.2)(&req)
        };

        res.headers_mut().set(header::ContentLength(corruption_response.body.as_bytes().len() as u64));
        res.headers_mut().set(header::Server("Corruption/0.1.0".to_owned()));
        res.headers_mut().set(corruption_response.content_type);

        let mut res = res.start().unwrap();
        res.write_all(corruption_response.body.as_bytes()).unwrap()

    }
}


impl Corruption {

    pub fn new() -> Corruption {
        Corruption { handler: MyHandler::new(), addr: "127.0.0.1:8080" }
    }

    fn route<T: 'static + Fn(&server::Request) -> self::response::Response  +Send+Sync>(&mut self, verb: method::Method, uri: &str, handler: T) {
        self.handler.routes.push( ( verb, uri.to_string(),Box::new(handler)) );
    }

    pub fn get<T: 'static + Fn(&server::Request) -> self::response::Response +Send+Sync>(&mut self, uri: &str, handler: T) -> &mut Corruption {
        self.route(method::Method::Get, uri, handler);
        self
    }

    pub fn listen(&mut self, addr: &'static str) -> &mut Corruption {
        self.addr = addr;
        self
    }

    pub fn serve(self) {
        server::Server::http(self.addr).unwrap().handle( self.handler ).unwrap();
    }
}
