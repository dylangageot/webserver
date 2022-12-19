use std::{
    io::{BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

use webserver::http::{index, Headers, Message, Method, Result, Status, Type};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        match handle_connection(stream) {
            Err(e) => eprintln!("{}", e),
            _ => continue,
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let request = Message::read(&mut BufReader::new(&mut stream))?;
    match request.message_type() {
        Type::Request { method, url, .. } => match method {
            Method::Get => {
                println!("Request: {:#?}", request);
                let response = Message::new(
                    Status::Ok,
                    Some(Headers::from([(
                        String::from("Content-Type"),
                        String::from("text/html"),
                    )])),
                    Some(index::display_dir(url)?),
                );
                //println!("Response: {:#?}", response);
                response.write(&mut BufWriter::new(&mut stream))?;
            }
            _ => {
                panic!("Only GET method is allowed for the moment");
            }
        },
        _ => {
            panic!("Received a response instead of a request...");
        }
    };
    Ok(())
}
