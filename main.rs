use secached::SeCached;
use std::{env, process::exit, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (port,mem,connections,listen) = parse_args(args);
    let secached = SeCached::new(port,mem,connections,listen);
    secached.start();
}

fn parse_args(args: Vec<String>) -> (u32,usize,u32,String) {
    let mut port: u32 = 11211;
    let mut mem: usize = 512 * 1024 * 1024; //512 Mb
    let mut connections: u32 = 64;
    let mut listen: String = "127.0.0.1".to_string();
    for (index, arg) in args.iter().enumerate() {
        if arg == "-p" {
            let parsed_port = args[index + 1].parse();
            match parsed_port {
                Ok(p) => port = p,
                Err(_) => {
                    println!("Error: Port should be an integer");
                    exit(1);
                }
            }
            if port < 1 || port > 65535 {
                println!("Error: Port should be between 1 and 65535");
                exit(1);
            }
        }
        if arg == "-m" {
            let parsed_mem = args[index + 1].parse();
            match parsed_mem {
                Ok(m) => mem = m,
                Err(_) => {
                    println!("Error: Memory should be an integer");
                    exit(1);
                }
            }
            if mem <= 0 {
                println!("Error: Memory should be more than zero MB");
                exit(1);
            }
        }
        if arg == "-c" {
            let parsed_connections = args[index + 1].parse();
            match parsed_connections {
                Ok(c) => connections = c,
                Err(_) => {
                    println!("Error: Connections' number should be an integer");
                    exit(1);
                }
            }
            if port < 1 || port > 65535 {
                println!("Error: Connections should be between 1 and 65535");
                exit(1);
            }
        }
        if arg == "-l" {
            let parsed_listen = args[index + 1].clone();
            if parsed_listen.is_empty() || !parsed_listen.parse::<std::net::IpAddr>().is_ok() {
                println!("Error: Invalid IP address for -l");
                exit(1);
            }
            listen = parsed_listen;
        }
    }
    (port,mem,connections,listen)
}
