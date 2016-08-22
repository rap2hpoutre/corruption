use request::Request;

enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}


struct Route {
    uri: String,
    verb: Method,
    handler: Box<Fn(Request)>
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    fn route<T: 'static + Fn(Request)>(&mut self, verb: Method, uri: &str, handler: T) {
        self.routes.push( Route { uri: uri.to_string(), verb: verb, handler: Box::new(handler) } )
    }
    
    pub fn get<T: 'static + Fn(Request)>(&mut self, uri: &str, handler: T) {
        self.route(Method::Get, uri, handler)
    }

    pub fn action(&self, i: usize, r: Request) {
        (*self.routes[i].handler)(r)
    }
}