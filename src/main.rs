use std::{
    io::{BufReader, BufWriter},
    net::{TcpListener, TcpStream},
    str::FromStr,
};

use webserver::http::{Body, Headers, Message, Method, Status, Type};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request = Message::read(&mut BufReader::new(&mut stream)).unwrap();
    match request.message_type() {
        Type::Request { method, url, .. } => match method {
            Method::Get => {
                println!("Request: {:#?}", request);
                let response = Message::new(
                    Status::Ok,
                    Some(Headers::from([(
                        String::from("Content-Type"),
                        String::from("text/plain"),
                    )])),
                    Some(Body::from_str(&format!("Content of: {}\n", url)).unwrap()),
                );
                println!("Response: {:#?}", response);
                response.write(&mut BufWriter::new(&mut stream)).unwrap();
            }
            _ => {
                eprintln!("Only GET method is allowed for the moment");
                return;
            }
        },
        _ => {
            eprintln!("Received a response instead of a request...");
            return;
        }
    };
}
