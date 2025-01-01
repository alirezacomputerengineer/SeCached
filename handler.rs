use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use crate::{parser::parse_req, Command, DataType, Request};
static WRONG_TYPE_ERROR_RESPONSE: &str =
    "-WRONGTYPE Operation against a key holding the wrong kind of value\r\n";

pub fn process_request(
    mut stream: TcpStream,
    cache: Arc<RwLock<HashMap<String, DataType>>>,
    bus: Arc<RwLock<HashMap<String, Vec<TcpStream>>>>,
    timeout: Option<Duration>,
) {
    loop {
        let cache = cache.clone();
        let bus = bus.clone();
        if let Some(timeout) = timeout {
            stream.set_read_timeout(Some(timeout)).unwrap();
        }

        let mut buf_reader = BufReader::new(&mut stream);
        let mut first_line = String::new();
        match buf_reader.read_line(&mut first_line) {
            Err(_) => {
                stream.flush().unwrap();
                stream.shutdown(std::net::Shutdown::Read).unwrap();
                return;
            }
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    stream.flush().unwrap();
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                    return;
                }
                let req = parse_req(&first_line); 
                
                // Process the request and get a response
                let response = get_response(cache, bus, &req, &mut stream);
                stream.write_all(response.as_bytes()).unwrap();
                
                // Handle QUIT command to close connection
                if req.command == Command::QUIT {
                    stream.flush().unwrap();
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                    return;
                }
            }
        }
    }
}


pub fn get_response(
    cache: Arc<RwLock<HashMap<String, DataType>>>,
    bus: Arc<RwLock<HashMap<String, Vec<TcpStream>>>>,
    req: &Request,
    stream: &mut TcpStream,
) -> String {
    match req.command {
        Command::SET => handle_set(req, cache),
        Command::ADD => handle_add(req, cache),
        Command::REPLACE => handle_replace(req, cache),
        Command::APPEND => handle_append(req, cache),
        Command::CAS => handle_cas(req, cache),
        Command::GET => handle_get(req, cache),
        Command::GETS => handle_gets(req, cache),
        Command::DELETE => handle_delete(req, cache),
        Command::INCR => handle_incr(req, cache),
        Command::DECR => handle_decr(req, cache),
        Command::STATS => handle_stats(req, cache),
        Command::STATS_ITEMS => handle_stat_items(req, cache),
        Command::STATS_SLABS => handle_stat_slabs(req, cache),
        Command::STATS_SIZES => handle_state_sizes(req, cache),
        Command::FLUSH_ALL => handle_flush_all(req, cache),
        Command::VERSION => handle_version(req, cache),
        Command::VERBOSITY => handle_verbosity(req, cache),
        Command::QUIT => handle_quit(),
    }
}

pub fn handle_set(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let mut cache = cache.write().unwrap();
    let value = req.value[0].clone();
    cache.insert(req.key.clone(), DataType::String(value));
    let response = "STORED\r\n".to_string();
    response
}

pub fn  handle_add(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_replace(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_append(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_cas(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_get(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let cache = cache.read().unwrap();
    let request_value = cache.get(&req.key);
    match request_value {
        Some(value) => match value {
            DataType::String(value) => {
                let response = format!("VALUE {} 0 {}\r\n{}\r\nEND", req.key, value.len(), value);
                return response;
            }

            _ => {
                let response = WRONG_TYPE_ERROR_RESPONSE.clone().to_string();
                return response;
            }
        },
        None => {
            let response = "END\r\n".to_string();
            return response;
        }
    }
}

pub fn  handle_gets(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_delete(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let mut cache = cache.write().unwrap();
    let key_to_delete = cache.get(&req.key);
    match key_to_delete {
        Some(_) => {
            cache.remove(&req.key);
            let response = "DELETED\r\n".to_string();
            response
        }
        None => {
            let response = "NOT_FOUND\r\n".to_string();
            response
        }
    }
}

pub fn  handle_incr(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_decr(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_stats(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_stat_items(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_stat_slabs(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_state_sizes(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_flush_all(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn  handle_version(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "0.0.1\r\n".to_string();
    response
}

pub fn  handle_verbosity(req: &Request, cache: Arc<RwLock<HashMap<String, DataType>>>) -> String {
    let response = "Not Implemented Yet !".to_string();
    response
}

pub fn handle_quit() -> String {
    "QUIT\r\n".to_string()
}
