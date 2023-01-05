async fn say_word() { // 需要用async（e,s',k）修饰。
    println!("my tokio");
}

#[tokio::main]
async fn main() {
    let op = say_word(); //被立即返回给op,而没有被执行，我们这时候叫op为一个Future(也就是现在为空，在未来才会产生的值)
    println!("hello");
    op.await;// 等待async异步操作执行完毕才返回，是一个阻塞操作
}
