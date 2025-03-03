use socktimized::ThreadPool;
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Socket is busy");
    let mut thread_pool = ThreadPool::new(8);
    listener
        .set_nonblocking(true)
        .expect("Failed to set the listener as nonblocking");

    println!("Starting server at: {}", "127.0.0.1:8000");

    for res_stream in listener.incoming() {
        match res_stream {
            Ok(stream) => {
                //stream
                //    .set_nonblocking(true)
                //    .expect("Failed to set stream as nonblocking");
                //std::thread::spawn(|| handle_tcp_stream(stream));
                thread_pool.execute(handle_tcp_stream, stream);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
            Err(err) => panic!("Error occured: {}", err),
        }
    }

    println!("Shutting down.")
}

fn handle_tcp_stream(mut stream: TcpStream) {
    stream.set_nodelay(true).expect("set_nodelay call failed");

    loop {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Err(_) => return,
            Ok(0) => return,
            Ok(_v) => {}
        }

        let response =
            b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\nConnection: keep-alive\r\n\r\nCHIYA";

        stream.write_all(response).unwrap();
    }
}
