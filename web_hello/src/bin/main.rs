use std::{fs, thread};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use web_hello::ThreadPool;

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // .take(2) 模拟只会接收2次通知就会结束循环
    //for stream in listener.incoming().take(2){
    for stream in listener.incoming(){
        let stream = stream.unwrap();

        // // 无脑开线程的方式，可能会被攻击
        // thread::spawn(||{
        //     handle_connection(stream);
        // });

        // 线程池
        pool.execute(||{
            handle_connection(stream);
        });

        println!("Shutting down.")
    }
}

fn handle_connection(mut stream:TcpStream){ //TcpStream的内部可能会改变，所以我们需要将它标记为mut.（一般情况确实不需要，这个地方是个例外）
    let mut  buffer = [0;512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep1 = b"GET /sleep HTTP/1.1\r\n";
    let (status_line,filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n",r"D:\zt\rust\hell\hello\web_hello\hello.html")
    }else if buffer.starts_with(sleep1){
        sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n",r"D:\zt\rust\hell\hello\web_hello\hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n",r"D:\zt\rust\hell\hello\web_hello\404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}",status_line,contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // 果然，这个地方还是要加点延时的，不然，文件可能压根传不出去。
    let three_seconds = Duration::from_millis(50); // 延时100ms
    sleep(three_seconds);
    println!("结束一次请求");
}