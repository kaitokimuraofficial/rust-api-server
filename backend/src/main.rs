extern crate postgres;

use postgres::{Client, Error, NoTls};


use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};



fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://postgres:password@127.0.0.1:5432",
        NoTls,
    )?;

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &mut client);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, client: & mut Client) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    if status_line == "HTTP/1.1 200 OK\r\n\r\n" {
        for row in client.query("SELECT id, name FROM users", &[]).unwrap() {
            let id: i32 = row.get(0);
            let name: String = row.get(1);
            println!("{} {}", id, name);
        }
    }else {
        println!("Invalid order");
    }

}