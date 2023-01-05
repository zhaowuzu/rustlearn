use tokio;

fn main(){
    // 创建runtime；这个方法默认的也是多线程
    // 默认工作线程数量和cpu核数（虚拟核，即CPU线程数）相同
    let rt = tokio::runtime::Runtime::new().unwrap();

    // 创建runtime；单线程
    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();

    // 创建带有线程池的runtime；线程即rust线程即os线程
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)// 8个工作线程
        .enable_io()// 可在异步io中使用异步io
        .enable_time()// 可在runtime中使用异步计时器（timer）
        .build()//创建runtime
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(30))
}