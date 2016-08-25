extern crate corruption;

use corruption::*;


fn main() {
    let mut c = Corruption::new();

    c.get("/test", |_,_| "str" );
    c.get("/test2", |req,res| { "str2" } );

    c.serve();
}
