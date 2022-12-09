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
    let request = match Request::from(&mut BufReader::new(&mut stream)) {
        Ok(h) => h,
        Err(s) => {
            eprintln!("Header parsing failed miserably {}", s);
            return;
        }
    };

    println!("Request: {:#?}", request);
}
