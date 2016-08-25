use corruption::*;


fn main() {
    let mut c = Corruption::new();

    c.get("/test", |_,_| "str" );
    c.get("/test2", |req,res| { let a = format!("str - {:?}", &req.method);  &a } );

    c.serve();
}
