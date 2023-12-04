use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use webweb::ThreadPool;

fn main() {
    //let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 7878)).expect("Failed to bind socket");
    let mut pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| handle_connection(stream)),
            Err(_) => println!("Ignoring failed connection attempt ((💀))"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let mut request_lines = buf_reader.lines();

    let first_line = request_lines.next().unwrap().unwrap();

    let _other_lines: Vec<_> = request_lines
        .map(|x| x.unwrap())
        .take_while(|x| !x.is_empty())
        .collect();

    println!("{first_line} from {:?}", stream.peer_addr());

    /*let (status_line, filename) = if first_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK BOOMER", "hello.html")
    } else {
        ("HTTP/1.1 404 how about NO", "404.html")
    };*/

    let (status_line, filename) = match first_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK BOOMER", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 I be gotten da sleepy", "hello.html")
        }
        _ => ("HTTP/1.1 404 how about NO", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    /*let mut html = File::open("hello.html").expect("Couldn't open hello.html");
    let mut html_buf = Vec::new();
    html.read_to_end(&mut html_buf).unwrap();

    stream.write_all("HTTP/1.1 200 OK BOOMER\r\n".as_bytes()).unwrap();
    stream.write_all(format!("Content-Length: {}\r\n\r\n", html_buf.len()).as_bytes()).unwrap();
    stream.write_all(&html_buf).unwrap();*/
}
