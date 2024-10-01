use std::net::{TcpListener, TcpStream, Shutdown};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

pub struct Server {
    /// Initializes a Server that will handle Client Requests. Store will be the key store in memory
    /// and workers will dictate how many threads can be opened at once.
    pub store: Arc<Mutex<HashMap<String, String>>>,
    pub workers: usize,
    pub port: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    /// This struct is for Deserializing the json object coming from the clients Request
    key: String,
    value: String,
}

impl Server {
    pub fn run(&self) -> std::io::Result<()> {
        // This will bind a TCPListener to port 3467 on the host. It will initialize a pool with
        // the servers worker amount. On a request stream it will spawn a new thread, close the Arc
        // Mutex responsible for keeping key store in sync, it will lock the store on entries. On
        // success it will shutdown the stream.
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr).map_err(|e| {
            eprintln!("Failed to bind to port {}: {}", self.port, e);
            e
        })?;
        let pool = rayon::ThreadPoolBuilder::new().num_threads(self.workers).build().unwrap();

        println!("Listening for Connections on port 3467");
        for stream in listener.incoming() {
            let store = Arc::clone(&self.store);
            pool.spawn(move || {
                let mut stream = stream.unwrap();
                println!("We unwrapped the stream");
                let pl = handle_client(&mut stream, store.clone()).expect("Protocol Error");
                let mut store_locked = store.lock().unwrap();
                store_locked.insert(pl.key, pl.value);
                println!("{:?}", *store_locked);
                stream.write(b"key").unwrap();
                stream.shutdown(Shutdown::Both).unwrap();
            });
        }
        Ok(())
    }
}


fn handle_client(stream: &mut TcpStream, _store: Arc<Mutex<HashMap<String, String>>>) -> Result<Payload, Box<dyn std::error::Error>> {
    let mut prefix_buf = [0; 10];
    stream.read(&mut prefix_buf).map_err(|e| {
        eprintln!("Failed to read prefix: {}", e);
        e
    })?;
    let prefix_string = String::from_utf8_lossy(&prefix_buf);
    let parts: Vec<&str> = prefix_string.split_whitespace().collect();
    let length: usize = parts[1].parse().map_err(|e| {
        eprintln!("Failed to parse length: {}", e);
        e
    })?;
    let mut buffer = vec![0; length];
    stream.read(&mut buffer).unwrap();
    let new_string = String::from_utf8_lossy(&buffer).to_string();
    let pl: Payload = serde_json::from_str(&new_string).map_err(|e| {
        eprintln!("Failed to Deserialize json: {}", e);
        e
    })?;
    Ok(pl)
}
