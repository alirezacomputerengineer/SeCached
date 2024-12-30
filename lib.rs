use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};
mod handler;
mod parser;
use handler::process_request;

pub struct SeCached {
    listener: TcpListener,
    port: u32,
    timeout: Option<Duration>,
}
impl SeCached {
    pub fn new(port: u32, timeout: Option<Duration>) -> SeCached {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).unwrap();
        SeCached {
            port,
            listener,
            timeout,
        }
    }

    pub fn start(&self) {
        println!("SeCached is running on port {}", self.port);
        let mut cache = Arc::new(RwLock::new(HashMap::new()));
        let bus = Arc::new(RwLock::new(HashMap::new()));
        let timeout = self.timeout;
        for stream in self.listener.incoming() {
            let cache = Arc::clone(&mut cache);
            let bus = Arc::clone(&bus);
            thread::spawn(move || {
                let stream = stream.unwrap();
                return process_request(stream, cache, bus, timeout);
            });
        }
    }
}
#[derive(Debug)]
pub enum Command {
    SET,
    ADD,
    REPLACE,
    APPEND,
    CAS,
    GET,
    GETS,
    DELETE,
    INCR,
    DECR,
    STATS,
    STATS_ITEMS,
    STATS_SLABS,
    STATS_SIZES,
    FLUSH_ALL,
    VERSION,
    VERBOSITY,
    QUIT,
}
#[derive(Debug)]
pub struct Request {
    command: Command,
    value: Vec<String>,
}

#[derive(Debug)]
pub enum DataType {
    String(String),
    List(Vec<String>),
    // Set,
    // Hash,
    // SortedSet,
}