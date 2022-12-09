use std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
};

use webserver::http::request::Request;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // TODO: find a way to get header clean, then the body
    let mut buf_reader = BufReader::new(&mut stream);

    let request = match Request::from(&mut buf_reader) {
        Ok(h) => h,
        Err(s) => {
            eprintln!("Header parsing failed miserably {}", s);
            return;
        }
    };

    println!("Request: {:#?}", request);
}
