use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, RwLock, atomic::{AtomicUsize, Ordering}},
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
    mem: u32,
    connections: u32,
    listen: String,
}
impl SeCached {
    pub fn new(port: u32, mem: u32, connections: u32, listen : String) -> SeCached {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).unwrap();
        SeCached {
            port,
            mem,
            connections,
            listen,
            listener,
        }
    }

    pub fn start(&self) {
        println!("SeCached is running on {}:{}", self.listen, self.port);
        let cache = Arc::new(RwLock::new(HashMap::new()));
        let bus = Arc::new(RwLock::new(HashMap::new()));
    
        // Track active connections using AtomicUsize
        let active_connections = Arc::new(AtomicUsize::new(0));
    
        for stream in self.listener.incoming() {
            let stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to accept a connection: {}", e);
                    continue;
                }
            };
    
            // Check if the connection is coming from the allowed IP
            let peer_ip = match stream.peer_addr() {
                Ok(addr) => addr.ip(),
                Err(_) => {
                    eprintln!("Failed to get peer address. Dropping connection.");
                    continue;
                }
            };
    
            if self.listen != "0.0.0.0" && peer_ip.to_string() != self.listen {
                eprintln!("Rejected connection from unauthorized IP: {}", peer_ip);
                continue;
            }
    
            // Check connection limit
            let current_connections = active_connections.fetch_add(1, Ordering::SeqCst);
            if current_connections >= self.connections as usize {
                eprintln!("Connection limit reached. Dropping connection from: {}", peer_ip);
                active_connections.fetch_sub(1, Ordering::SeqCst);
                continue;
            }
    
            let cache = Arc::clone(&cache);
            let bus = Arc::clone(&bus);
            let active_connections = Arc::clone(&active_connections);
    
            thread::spawn(move || {
                let result = process_request(stream, cache, bus);
                active_connections.fetch_sub(1, Ordering::SeqCst); // Decrement active connection count
                result
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
    PREPEND,
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
