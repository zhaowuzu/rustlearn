use std::fs;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

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

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get){
        // println!("Request:{}",String::from_utf8_lossy(&buffer[..]));
        // 响应
        let contents = fs::read_to_string(r"D:\zt\rust\hell\hello\web_hello\hello.html").unwrap();
        //println!("{}",contents);
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type=text/html;charset=utf-8\r\n\r\n{}",contents);
        //let response = format!("HTTP/1.1 200 OK\r\n\r\n{}","contents"); // 总是没有办法正确出现页面信息
        //println!("{}",response);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        // println!("Request:{}",String::from_utf8_lossy(&buffer[..]));

        // 果然，这个地方还是要加点延时的，不然，文件可能压根传不出去。
        let three_seconds = Duration::from_millis(50); // 延时100ms
        sleep(three_seconds);
        println!("结束一次请求");
    }else{
        // 一些其他的请求
        println!("非浏览器请求");
    }
}