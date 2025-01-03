# SeCached: A Secure and Efficient Caching System

SeCached is a high-performance, secure, and minimalistic caching system inspired by Memcached, implemented in Rust. The project leverages Rust's safety and concurrency features to provide a robust and efficient caching solution.

## Features
- **Lightweight and Fast**: Designed to handle high-throughput scenarios with minimal latency.
- **Secure by Design**: Built using Rust for memory safety and thread safety.
- **Protocol Compatibility**: Implements the Memcached ASCII protocol for compatibility with existing tools and libraries.
- **Customizable**: Supports runtime configuration for memory allocation, threading, and more.
- **Extensible**: Easy to add new features or custom commands.

## Why SeCached?
- **Secure**: Avoids common vulnerabilities like buffer overflows, thanks to Rust's safety guarantees.
- **Efficient**: Optimized for performance and low resource usage.
- **Open Source**: Contributions are welcome to help build a robust alternative to Memcached.

## Getting Started
### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/alirezacomputerengineer/seCached.git
   cd seCached
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the server:
   ```bash
   ./target/release/seCached -p 11211
   ```

### Usage
SeCached supports the Memcached ASCII protocol. You can interact with it using any Memcached client or via telnet:
```bash
telnet 127.0.0.1 11211
```
### Example:
```bash
set mykey 0 900 5 hello

get mykey
```

## TODO Table
### Commands Implementation
| **Command**       | **Description**                               | **Example**                          | **Status** |
|--------------------|-----------------------------------------------|---------------------------------------|------------|
| `set`             | Store a value with a key.                    | `set mykey 0 900 5 hello\r\n`   |     ✔      |
| `add`             | Store a value if the key doesn’t exist.     | `add mykey 0 900 5 hello\r\n`   |     ✔      |
| `replace`         | Replace the value of an existing key.        | `replace mykey 0 900 5 world\r\n` |     ✔      |
| `append`          | Append data to an existing value.            | `append mykey 0 900 5 there\r\n` |     ✔      |
| `prepend`         | Prepend data to an existing value.           | `prepend mykey 0 900 5 Hi \r\n` |     ✔      |
| `cas`             | Store a value if the CAS token matches.      | `cas mykey 0 900 5 42 hello\r\n` |            |
| `get`             | Retrieve a value by its key.                 | `get mykey\r\n`                    |     ✔      |
| `gets`            | Retrieve a value with its CAS token.         | `gets mykey\r\n`                   |            |
| `delete`          | Delete a key and its value.                  | `delete mykey\r\n`                 |     ✔      |
| `incr`            | Increment a numeric value.                   | `incr mycounter 5\r\n`             |            |
| `decr`            | Decrement a numeric value.                   | `decr mycounter 3\r\n`             |            |
| `stats`           | Get server statistics.                       | `stats\r\n`                        |            |
| `stats items`     | Get statistics about item storage.           | `stats items\r\n`                  |            |
| `stats slabs`     | Get statistics about slab allocation.        | `stats slabs\r\n`                  |            |
| `stats sizes`     | Get statistics about item sizes.             | `stats sizes\r\n`                  |            |
| `flush_all`       | Clear all data from the cache.               | `flush_all\r\n`                    |            |
| `version`         | Get the server version.                      | `version\r\n`                      |     ✔      |
| `verbosity`       | Set the verbosity level.                     | `verbosity 2\r\n`                  |            |
| `quit`            | Close the connection.                        | `quit\r\n`                         |     ✔      |

### Runtime Options Implementation
| **Option**         | **Description**                              | **Example**                  | **Status** |
|--------------------|----------------------------------------------|------------------------------|------------|
| `-m <size>`       | Maximum memory usage (MB).                  | `-m 64`                     |            |
| `-p <port>`       | Port number.                                | `-p 11211`                  |      ✔     |
| `-l <ip>`         | Listen on specific IP address.              | `-l 127.0.0.1`              |            |
| `-c <connections>`| Max simultaneous connections.               | `-c 1024`                   |            |
| `-d`              | Run as a daemon.                            | `-d`                        |            |
| `-u <user>`       | User to run the server as.                  | `-u memcache`               |            |
| `-t <threads>`    | Number of threads for request handling.      | `-t 4`                      |            |
| `-I <size>`       | Max item size (default: 1MB).               | `-I 2m`                     |            |
| `-o slab_reassign`| Enable slab reassign.                        | `-o slab_reassign`          |            |
| `-o slab_automove`| Enable automatic slab memory management.     | `-o slab_automove`          |            |
| `-k`              | Lock memory to prevent swapping.             | `-k`                        |            |
| `-v`              | Enable verbose output.                      | `-v`                        |            |
| `-P <file>`       | Store PID file path.                        | `-P /var/run/seCached.pid`  |            |
| `-d <protocol>`   | Use specific protocol (tcp/udp).            | `-d tcp`                    |            |
| `-a <permissions>`| Set UNIX socket permissions.                | `-a 770`                    |            |
| `-s <socket>`     | Use UNIX socket instead of TCP.             | `-s /tmp/seCached.sock`     |            |

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## License
SeCached is licensed under the MIT License.

