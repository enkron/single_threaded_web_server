use std::net::TcpListener;

// Listen at the addr 127.0.0.1:7878 for incoming TCP streams.
fn main() {
    // the `bind` func in TcpListener acts like `new` func:
    // it returns a new TcpListener instance.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for _stream in listener.incoming() {
        let _stream = _stream.unwrap();

        println!("Connection established.");
    }
}
