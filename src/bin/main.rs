use key_store_lite::server::Server;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() {
    let store = Arc::new(Mutex::new(HashMap::new()));
    let server = Server { store, workers: 4, port: 3467 };

    if let Err(e) = server.run() {
        eprintln!("Server failed: {}", e);
    }
}
