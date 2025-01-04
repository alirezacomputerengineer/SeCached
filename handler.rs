use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{parser::parse_req, Command, DataType, Request, CacheItem};
static WRONG_TYPE_ERROR_RESPONSE: &str =
    "-WRONGTYPE Operation against a key holding the wrong kind of value\r\n";

pub fn process_request(
    mut stream: TcpStream,
    cache: Arc<RwLock<HashMap<String, CacheItem>>>,
    bus: Arc<RwLock<HashMap<String, Vec<TcpStream>>>>,
    mem: usize,
) {
    loop {
        let cache = cache.clone();
        let bus = bus.clone();

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
                let response = get_response(cache, bus, &req, &mut stream, mem);
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
    cache: Arc<RwLock<HashMap<String, CacheItem>>>,
    bus: Arc<RwLock<HashMap<String, Vec<TcpStream>>>>,
    req: &Request,
    stream: &mut TcpStream,
    mem: usize,
) -> String {
    match req.command {
        Command::SET => handle_set(req, cache, mem),
        Command::ADD => handle_add(req, cache, mem),
        Command::REPLACE => handle_replace(req, cache, mem),
        Command::APPEND => handle_append(req, cache, mem),
        Command::PREPEND => handle_prepend(req, cache, mem),
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
        Command::VERSION => handle_version(),
        Command::QUIT => handle_quit(),
        Command::ERROR => handle_error(),
    }
}

pub fn handle_set(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>, mem: usize) -> String {
    if req.value.len() != 4 {
        return "CLIENT_ERROR bad command line format\r\n".to_string();
    }

    let flags: u32 = req.value[0].parse().unwrap_or(0);
    let exptime: u64 = req.value[1].parse().unwrap_or(0);
    let bytes: usize = req.value[2].parse().unwrap_or(0);
    let data = req.value[3].clone();

    if data.len() != bytes {
        return "CLIENT_ERROR bad data chunk\r\n".to_string();
    }

    let expiration = if exptime == 0 {
        0
    } else {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        now + exptime
    };

    let cache_item = CacheItem {
        data_type: DataType::String(data.clone()),
        flags,
        expiration,
        size: bytes,
        created_at: SystemTime::now(),
    };

    let mut cache = cache.write().unwrap();

    if !ensure_memory(&mut cache, mem, bytes) {
        return "SERVER_ERROR not enough memory\r\n".to_string();
    }

    cache.insert(req.key.clone(), cache_item);
    "STORED\r\n".to_string()
}

pub fn handle_add(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>, mem: usize) -> String {
    if req.value.len() != 4 {
        return "CLIENT_ERROR bad command line format\r\n".to_string();
    }

    let flags: u32 = req.value[0].parse().unwrap_or(0);
    let exptime: u64 = req.value[1].parse().unwrap_or(0);
    let bytes: usize = req.value[2].parse().unwrap_or(0);
    let data = req.value[3].clone();

    if data.len() != bytes {
        return "CLIENT_ERROR bad data chunk\r\n".to_string();
    }

    let mut cache = cache.write().unwrap();

    if cache.contains_key(&req.key) {
        return "NOT_STORED\r\n".to_string();
    }

    let expiration = if exptime == 0 {
        0
    } else {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        now + exptime
    };

    let cache_item = CacheItem {
        data_type: DataType::String(data.clone()),
        flags,
        expiration,
        size: bytes,
        created_at: SystemTime::now(),
    };

    if !ensure_memory(&mut cache, mem, bytes) {
        return "SERVER_ERROR not enough memory\r\n".to_string();
    }

    cache.insert(req.key.clone(), cache_item);
    "STORED\r\n".to_string()
}

pub fn handle_replace(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>, mem: usize) -> String {
    if req.value.len() != 4 {
        return "CLIENT_ERROR bad command line format\r\n".to_string();
    }

    let flags: u32 = req.value[0].parse().unwrap_or(0);
    let exptime: u64 = req.value[1].parse().unwrap_or(0);
    let bytes: usize = req.value[2].parse().unwrap_or(0);
    let data = req.value[3].clone();

    if data.len() != bytes {
        return "CLIENT_ERROR bad data chunk\r\n".to_string();
    }

    let mut cache = cache.write().unwrap();

    if !cache.contains_key(&req.key) {
        return "NOT_STORED\r\n".to_string();
    }

    let expiration = if exptime == 0 {
        0
    } else {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        now + exptime
    };

    let cache_item = CacheItem {
        data_type: DataType::String(data.clone()),
        flags,
        expiration,
        size: bytes,
        created_at: SystemTime::now(),
    };

    if !ensure_memory(&mut cache, mem, bytes) {
        return "SERVER_ERROR not enough memory\r\n".to_string();
    }

    cache.insert(req.key.clone(), cache_item);
    "STORED\r\n".to_string()
}

pub fn handle_append(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>, mem: usize) -> String {
    if req.value.len() != 4 {
        return "CLIENT_ERROR bad command line format\r\n".to_string();
    }

    let bytes: usize = req.value[2].parse().unwrap_or(0);
    let data = req.value[3].clone();

    if data.len() != bytes {
        return "CLIENT_ERROR bad data chunk\r\n".to_string();
    }

    let mut cache = cache.write().unwrap();

    if let Some(existing_item) = cache.get_mut(&req.key) {
        // Temporarily extract the data type to avoid simultaneous mutable borrows
        if let DataType::String(existing_data) = &mut existing_item.data_type {
            let new_size = existing_data.len() + bytes;
            let size_diff = new_size - existing_item.size;
    
            // Drop the mutable borrow of existing_item before calling ensure_memory
            drop(existing_item);
    
            // Perform the memory check
            if !ensure_memory(&mut *cache, mem, size_diff) {
                return "SERVER_ERROR not enough memory\r\n".to_string();
            }
    
            // Re-borrow the cache to update the item
            if let Some(existing_item) = cache.get_mut(&req.key) {
                if let DataType::String(existing_data) = &mut existing_item.data_type {
                    existing_data.push_str(&data); // For append
                    existing_item.size = new_size;
                    return "STORED\r\n".to_string();
                }
            }
    
            // This should not happen, but handle it gracefully
            return "SERVER_ERROR internal error\r\n".to_string();
        } else {
            return "CLIENT_ERROR incompatible data type\r\n".to_string();
        }
    } else {
        return "NOT_STORED\r\n".to_string();
    }        
}

pub fn handle_prepend(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>, mem: usize) -> String {
    if req.value.len() != 4 {
        return "CLIENT_ERROR bad command line format\r\n".to_string();
    }

    let bytes: usize = req.value[2].parse().unwrap_or(0);
    let data = req.value[3].clone();

    if data.len() != bytes {
        return "CLIENT_ERROR bad data chunk\r\n".to_string();
    }

    let mut cache = cache.write().unwrap();

    if let Some(existing_item) = cache.get_mut(&req.key) {
        // Temporarily extract the data type to avoid simultaneous mutable borrows
        if let DataType::String(existing_data) = &mut existing_item.data_type {
            let new_size = existing_data.len() + bytes;
            let size_diff = new_size - existing_item.size;
    
            // Drop the mutable borrow of existing_item before calling ensure_memory
            drop(existing_item);
    
            // Perform the memory check
            if !ensure_memory(&mut *cache, mem, size_diff) {
                return "SERVER_ERROR not enough memory\r\n".to_string();
            }
    
            // Re-borrow the cache to update the item
            if let Some(existing_item) = cache.get_mut(&req.key) {
                if let DataType::String(existing_data) = &mut existing_item.data_type {
                    existing_data.push_str(&data); // For append
                    existing_item.size = new_size;
                    return "STORED\r\n".to_string();
                }
            }
    
            // This should not happen, but handle it gracefully
            return "SERVER_ERROR internal error\r\n".to_string();
        } else {
            return "CLIENT_ERROR incompatible data type\r\n".to_string();
        }
    } else {
        return "NOT_STORED\r\n".to_string();
    }        
}

pub fn  handle_cas(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>) -> String {
    "Not Implemented Yet !\r\n".to_string()
}

pub fn handle_get(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>) -> String {
    let mut cache = cache.write().unwrap();

    // Check if the key exists in the cache
    if let Some(item) = cache.get(&req.key) {
        // Check if the item has expired
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        if item.expiration != 0 && now > item.expiration {
            // Remove expired item
            cache.remove(&req.key);
            return "END\r\n".to_string();
        }

        // Handle different data types
        match &item.data_type {
            DataType::String(value) => {
                let response = format!(
                    "VALUE {} {} {}\r\n{}\r\nEND\r\n",
                    req.key, item.flags, value.len(), value
                );
                return response;
            }
            _ => {
                // Return error for unsupported data type
                return "CLIENT_ERROR wrong type\r\n".to_string();
            }
        }
    }

    // Key not found
    "END\r\n".to_string()
}

pub fn  handle_gets(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>) -> String {
    "Not Implemented Yet !\r\n".to_string()
}

pub fn handle_delete(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>) -> String {
    let mut cache = cache.write().unwrap();

    if cache.remove(&req.key).is_some() {
        "DELETED\r\n".to_string()
    } else {
        "NOT_FOUND\r\n".to_string()
    }
}

pub fn handle_incr(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>) -> String {
    // Ensure the request has exactly 2 parts: key and increment value
    if req.value.len() != 1 {
        return "CLIENT_ERROR bad command line format\r\n".to_string();
    }

    // Parse the increment value
    let incr_value: u64 = req.value[0].parse().unwrap_or(0); // Default to 0 if parsing fails

    // Lock the cache for writing
    let mut cache = cache.write().unwrap();

    // Find the key in the cache
    if let Some(existing_item) = cache.get_mut(&req.key) {
        if let DataType::String(existing_data) = &mut existing_item.data_type {
            if let Ok(current_value) = existing_data.parse::<u64>() {
                // Increment the value
                let new_value = current_value + incr_value;
                *existing_data = new_value.to_string();

                return format!("{}\r\n", new_value);
            } else {
                return "CLIENT_ERROR cannot increment non-numeric value\r\n".to_string();
            }
        } else {
            return "CLIENT_ERROR incompatible data type\r\n".to_string();
        }
    } else {
        return "NOT_FOUND\r\n".to_string();
    }
}

pub fn handle_decr(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>) -> String {
    // Ensure the request has exactly 2 parts: key and decrement value
    if req.value.len() != 1 {
        return "CLIENT_ERROR bad command line format\r\n".to_string();
    }

    // Parse the decrement value
    let decr_value: u64 = req.value[0].parse().unwrap_or(0); // Default to 0 if parsing fails

    // Lock the cache for writing
    let mut cache = cache.write().unwrap();

    // Find the key in the cache
    if let Some(existing_item) = cache.get_mut(&req.key) {
        if let DataType::String(existing_data) = &mut existing_item.data_type {
            if let Ok(current_value) = existing_data.parse::<u64>() {
                // Decrement the value, ensuring it does not go below 0
                let new_value = current_value.saturating_sub(decr_value);
                *existing_data = new_value.to_string();

                return format!("{}\r\n", new_value);
            } else {
                return "CLIENT_ERROR cannot decrement non-numeric value\r\n".to_string();
            }
        } else {
            return "CLIENT_ERROR incompatible data type\r\n".to_string();
        }
    } else {
        return "NOT_FOUND\r\n".to_string();
    }
}

pub fn  handle_stats(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>) -> String {
    "Not Implemented Yet !\r\n".to_string()
}

pub fn  handle_stat_items(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>) -> String {
    "Not Implemented Yet !\r\n".to_string()
}

pub fn  handle_stat_slabs(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>) -> String {
    "Not Implemented Yet !\r\n".to_string()
}

pub fn  handle_state_sizes(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>) -> String {
    "Not Implemented Yet !\r\n".to_string()
}

pub fn handle_flush_all(req: &Request, cache: Arc<RwLock<HashMap<String, CacheItem>>>) -> String {
    // Parse optional delay from the request
    let delay = if !req.value.is_empty() {
        req.value[0].parse::<u64>().unwrap_or(0) // Default to 0 if parsing fails
    } else {
        0
    };

    if delay == 0 {
        // Immediate flush: Clear the cache
        let mut cache = cache.write().unwrap();
        cache.clear();
    } else {
        // Delayed flush: Schedule the cache clearing
        let cache_clone = Arc::clone(&cache);
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(delay));
            let mut cache = cache_clone.write().unwrap();
            cache.clear();
        });
    }

    // Return response
    "OK\r\n".to_string()
}

pub fn  handle_version() -> String {
    "0.0.1\r\n".to_string()
}

pub fn handle_quit() -> String {
    "QUIT\r\n".to_string()
}

pub fn handle_error() -> String {
    "ERROR\r\n".to_string()
}

fn ensure_memory(cache: &mut HashMap<String, CacheItem>, mem_limit: usize, new_item_size: usize) -> bool {
    let current_mem_usage: usize = cache.values().map(|item| item.size).sum();

    if current_mem_usage + new_item_size <= mem_limit {
        return true;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    cache.retain(|_, item| item.expiration == 0 || item.expiration > now);

    let updated_mem_usage: usize = cache.values().map(|item| item.size).sum();
    updated_mem_usage + new_item_size <= mem_limit
}
