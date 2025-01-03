use std::collections::HashSet;

use crate::{Command, Request};

pub fn parse_req(s: &String) -> Request {
    let lines: Vec<&str> = s.split("\r\n").collect();
    if lines.len() < 1 {
        panic!("Invalid request format");
    }

    let first_line = lines[0];
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.is_empty() {
        panic!("Empty command");
    }
    let mut command: Option<Command> = None;
    let mut key = String::new();
    let mut value: Vec<String> = vec![];
    match parts[0].to_uppercase().as_str() {
        "SET" => command = Some(Command::SET),
        "ADD" => command = Some(Command::ADD),
        "REPLACE" => command = Some(Command::REPLACE),
        "APPEND" => command = Some(Command::APPEND),
        "PREPEND" => command = Some(Command::PREPEND),
        "CAS" => command = Some(Command::CAS),
        "GET" => command = Some(Command::GET),
        "GETS" => command = Some(Command::GETS),
        "DELETE" => command = Some(Command::DELETE),
        "INCR" => command = Some(Command::INCR),
        "DECR" => command = Some(Command::DECR),
        "STATS" => command = Some(Command::STATS),
        "STATS ITEMS" => command = Some(Command::STATS_ITEMS),
        "STATS SLABS" => command = Some(Command::STATS_SLABS),
        "STATS SIZES" => command = Some(Command::STATS_SIZES),
        "FLUSH_ALL" => command = Some(Command::FLUSH_ALL),
        "VERSION" => command = Some(Command::VERSION),
        "QUIT" => command = Some(Command::QUIT),
        _ => command = Some(Command::ERROR),
    };

    key = if parts.len() > 1 { parts[1].to_string() } else { "".to_string() };
    if parts.len() > 2 {
    value = parts[2..]
        .iter()
        .map(|part| part.to_string())
        .collect::<Vec<String>>();
    }
    let req = Request {
        command: command.expect("Expected a valid command"),
        key,
        value,
    };
    return req
}

