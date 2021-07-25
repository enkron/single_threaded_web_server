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

    let contents = fs::read_to_string("hello.html").unwrap();

    // holds success message's data
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Lenght: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    // convert the bytes in the buffer to a string and print it
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // Response
    stream.write(response.as_bytes()).unwrap(); // sends bytes directly
                                                // down the connection
    stream.flush().unwrap(); // prevent the program from continuing until
                             // all the bytes are written to the connection.
}
