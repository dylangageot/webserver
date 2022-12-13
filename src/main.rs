use std::{
    io::{BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

use webserver::http::{Headers, Message, Status, Body};

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

    let response = Message::new(
        Status::Ok,
        Some(Headers::new()),
        Some(Body::from("Hello world")),
    );
    println!("Response: {:#?}", response);
    response.to(&mut BufWriter::new(&mut stream)).unwrap();
}
