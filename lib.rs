use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
    time::SystemTime,
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
#[derive(Debug, PartialEq)]
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
    ERROR, 
}
#[derive(Debug)]
pub struct Request {
    command: Command,
    key: String,
    value: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    List(Vec<String>),
    Set(Vec<String>),
    Hash(std::collections::HashMap<String, String>),
    SortedSet(Vec<(String, i32)>), // Example: tuples of (key, score)
}

#[derive(Debug, Clone)]
pub struct CacheItem {
    pub data_type: DataType, // The type of data (String, List, etc.)
    pub flags: u32,          // Metadata about the data
    pub expiration: u64,     // Expiration time as a UNIX timestamp (0 means never expires)
    pub size: usize,         // Size of the data in bytes
    pub created_at: SystemTime, // Timestamp of when the item was created
}
