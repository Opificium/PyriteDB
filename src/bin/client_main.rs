use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:7878";
    let stream = TcpStream::connect(addr)?;
    println!("Connected to {addr}. Commands: GET/SET/DEL, 'quit' to exit application");

    //Two separate handles on same socket; one to write, one (wrapped in BufReader) to read answers line-wise
    let mut writer = stream.try_clone()?;
    let mut server_reader = BufReader::new(stream);

    let stdin = io::stdin();
    let mut input_line = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        input_line.clear();
        let bytes_read = stdin.lock().read_line(&mut input_line)?;
        if bytes_read == 0 {
            // Control D or EOF
            break;
        }

        let command = input_line.trim();

        if command.is_empty() {
            continue;
        }
        if command.eq_ignore_ascii_case("quit") {
            break;
        }

        writeln!(writer, "{command}")?;

        let mut response = String::new();
        let n = server_reader.read_line(&mut response)?;
        if n == 0 {
            println!("Server closed the connection.");
            break;
        }
        print!("{response}");
    }
    println!("Connection terminated.");
    Ok(())
}