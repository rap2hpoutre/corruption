# Corruption
Undocumented rust web framework made with :footprints:

## Usage
Do not use. Dev in progress.

```rust
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

    // Serve it to the world on 127.0.0.1:8080
    corruption.serve();
}
```
