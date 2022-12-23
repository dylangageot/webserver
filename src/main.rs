use std::{
    io::{BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

use webserver::http::{index, Message, Method, Result, StartLine};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        match handle_connection(stream) {
            Err(e) => eprintln!("{:#?}", e),
            _ => continue,
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    const BASE_PATH: &str = "/home/gageotd";
    let request = Message::read(&mut BufReader::new(&mut stream))?;
    match request.start_line() {
        StartLine::Request { method, url, .. } => match method {
            Method::Get => {
                println!("Request: {:#?}", request);
                index::generate(BASE_PATH, url)?.write(&mut BufWriter::new(&mut stream))?;
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
