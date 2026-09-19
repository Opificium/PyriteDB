use super::db::Db;
use super::protocol::{execute, parse_line};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

/// Handles a single client connection, reads each line, parses, executes against db,
/// and responds with answer
fn handle_client(stream: TcpStream, db: Arc<Db>) {
    let peer = stream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    println!("[+] New connection: {peer}");

    let reader = BufReader::new(
        stream
            .try_clone()
            .expect("Stream could not be cloned.")
    );

    let mut writer = stream;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprint!("[!] Error reading from {peer}: {e}");
                break;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let response = match parse_line(&line) {
            Ok(cmd) => execute(&db, cmd),
            Err(e) => format!("ERROR {e}")
        };

        if let Err(e) = writeln!(writer, "{response}") {
            eprintln!("[!] Error writing to {peer}: {e}");
            break;
        }
    }

    println!("[-] Connection closed: {peer}");
}

/// Starts TCP server and blocks, until process is finished
pub fn run(addr: &str, db: Arc<Db>) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("KV server is listening on {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_clone = Arc::clone(&db);
                thread::spawn(move || {
                    handle_client(stream, db_clone);
                });
            }
            Err(e) => eprintln!("[!] Connection error: {e}")
        }
    }
    Ok(())
}