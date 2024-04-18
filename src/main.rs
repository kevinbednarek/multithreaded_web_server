use std::{fs, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn main() {
    //TODO: Add error handling if port is already in use
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //TODO: Print error message if stream produces connection attempt instead of actual connection

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        //TODO: Error handling. This comes back as None sometimes..
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        /*let http_request: Vec<_> = buf_reader //TODO: implement better error handling
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();*/


        //println!("Request: {:#?}", http_request); //TODO: Implement this as logging..?

        let (status_line, file_name) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            },
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        /*let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };*/

        let contents = fs::read_to_string(file_name).unwrap(); //TODO: error handling
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap(); //TODO: Implement error handling
    }
}
