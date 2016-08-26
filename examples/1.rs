extern crate corruption;

use corruption::Corruption;
use corruption::response::Response;


fn main() {
    // Start Corruption
    let mut corruption = Corruption::new();

    // Declare routes
    corruption
        .get("/test", |_| Response::html_str("fefddfd") )
        .get("/test2", |_| Response::html_str("fefddfd2") );

    // Serve it to the world
    corruption.serve();
}
