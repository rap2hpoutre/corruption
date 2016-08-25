extern crate hyper;
use std::io::{Write};
use hyper::*;

pub type Route = (method::Method, String, Box<Fn(&server::Request, &server::Response) -> &'static str + Send + Sync>);

pub struct Corruption {
    handler: MyHandler
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
                println!("FOUND!");
                route = Some(i);
            }
        }

        let body: &str = match route {
            None => "<h1>Not Found</h1>",
            Some(r) => (r.2)(&req, &res)
        };

        res.headers_mut().set(header::ContentLength(body.as_bytes().len() as u64));
        res.headers_mut().set(header::Server("Corruption/0.1.0".to_owned()));
        res.headers_mut().set(header::ContentType(mime::Mime(mime::TopLevel::Text, mime::SubLevel::Html, vec![(mime::Attr::Charset, mime::Value::Utf8)])));

        let mut res = res.start().unwrap();
        res.write_all(body.as_bytes()).unwrap()

    }
}


impl Corruption {

    pub fn new() -> Corruption {
        Corruption { handler: MyHandler::new() }
    }

    fn route<T: 'static + Fn(&server::Request, &server::Response) -> &'static str  +Send+Sync>(&mut self, verb: method::Method, uri: &str, handler: T) {
        self.handler.routes.push( ( verb, uri.to_string(),Box::new(handler)) )
    }

    pub fn get<T: 'static + Fn(&server::Request, &server::Response) -> &'static str +Send+Sync>(&mut self, uri: &str, handler: T) {
        self.route(method::Method::Get, uri, handler)
    }

    pub fn serve(self) {
        server::Server::http("127.0.0.1:8080").unwrap().handle( self.handler ).unwrap();
    }
}
