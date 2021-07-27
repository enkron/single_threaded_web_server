/// Hand-coded an http request/response
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

// Listen at the addr 127.0.0.1:7878 for incoming TCP streams.
fn main() {
    // the `bind` func in TcpListener acts like `new` func:
    // it returns a new TcpListener instance.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // `incoming` method of TcpListener returns an iterator
    // that gives us a sequence of TcpStream type streams.
    // A single stream represents an open client <--> server connection.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // declare a buffer on the stack

    // Request
    stream.read(&mut buffer).unwrap(); // read bytes from TcpStream and
                                       // put them in the buffer

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap(); // sends bytes directly
                                                // down the connection
    stream.flush().unwrap(); // prevent the program from continuing until
                             // all the bytes are written to the connection.
}
