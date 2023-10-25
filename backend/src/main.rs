extern crate postgres;
use postgres::{Client, Error, NoTls};


use std::{ fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};


fn main() {
    let mut client = Client::connect(
        "postgresql://postgres:password@127.0.0.1:5432/testdb",
        NoTls,
    ).unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &mut client);
    }
}

fn handle_connection(mut stream: TcpStream, client: & mut Client) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        for row in client.query("SELECT id, name FROM users", &[]).unwrap() {
            let id: i32 = row.get(0);
            let name: String = row.get(1);
            println!("{} {}", id, name);
        }
    }

}