use std::collections::HashSet;

use crate::{Command, Request};

pub fn parse_resp(s: &String) -> Request {
    let mut splitted: Vec<&str> = s.split("\r\n").collect();
    splitted.remove(splitted.len() - 1);
    let size = splitted[0][1..].parse::<usize>().unwrap();
    let mut command: Option<Command> = None;
    let mut key = String::new();
    let mut value: Vec<String> = vec![];
    for i in 1..=size * 2 {
        if i % 2 == 1 {
            continue;
        }
        if i == 2 {
            match splitted[i].to_uppercase().as_str() {
                "SET" => command = Some(Command::SET),
                "ADD" => command = Some(Command::ADD),
                "REPLACE" => command = Some(Command::REPLACE),
                "APPEND" => command = Some(Command::APPEND),
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
                "VERBOSITY" => command = Some(Command::VERBOSITY),
                "QUIT" => command = Some(Command::QUIT),
                other => {
                    panic!("ERROR");
                }
            }
        } else if i == 4 {
            key = splitted[i].into();
        } else {
            value.push(splitted[i].into())
        }
    }
    let req = Request {
        command: command.expect("Expected a valid command"),
        key,
        value,
    };
    return req
}
