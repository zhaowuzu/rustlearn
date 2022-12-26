use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);
    }// stream会在这个地方结束声明周期
}

fn handle_connection(mut stream:TcpStream){ //TcpStream的内部可能会改变，所以我们需要将它标记为mut.（一般情况确实不需要，这个地方是个例外）
    let mut  buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    println!("Request:{}",String::from_utf8_lossy(&buffer[..]));
}