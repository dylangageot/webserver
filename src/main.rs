use std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
};

use webserver::http::message::Message;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request = Message::from(&mut BufReader::new(&mut stream)).unwrap();
    println!("Request: {:#?}", request);
}
