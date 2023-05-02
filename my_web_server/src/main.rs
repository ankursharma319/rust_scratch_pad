use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::fs;
use std::thread;
use std::time::Duration;
use my_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind(("localhost",7878)).expect("Failed to bind tcp socket for listening");
    println!("Hello, world!");
    let thread_pool = ThreadPool::new(3);
    // automatically shut down server after 2 requests
    for connection_stream in listener.incoming().take(2) {
        let stream = connection_stream.expect("Failed to connect to socket");
        println!("Connection established!");
        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // println!("Received HTTP request: {:#?}", http_request);
    let first_line : &String = http_request.first().expect("Did not get even one line http request");

    let (status_line, file_name) = match first_line.as_str() {
        "GET / HTTP/1.1" => { ("HTTP/1.1 200 OK", "hello.html") },
        "GET /sleep HTTP/1.1" => {
            // println!("Sleeping for 5 seconds to simulate a slow response");
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(file_name).expect("Failed to read html file");
    let headers = format!("Content-Length: {}\r\n", contents.len());
    let response = format!("{}\r\n{}\r\n{}", status_line, headers, contents);
    // println!("Responding with response:\n{}", response);
    stream.write_all(response.as_bytes()).expect("Failed to write response to tcp stream");
}
