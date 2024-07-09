use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use http::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Connected to 127.0.0.1:7878");

    // Gonna add this of course
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // First we read input by making a Buffer for the streaming data
    let buf_reader = BufReader::new(&mut stream);

    // Parse Request Line from buffer reader
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // If a good request then return status/filename for all others fail
    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "http.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "http.html")
        }

        _ => ("HTTP/1.1 404 Bad Request", "404.html"),
    };

    // Load up the contents by reading the custom html file
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Write the stream data as a response to the client :D
    stream.write_all(response.as_bytes()).unwrap();
}
