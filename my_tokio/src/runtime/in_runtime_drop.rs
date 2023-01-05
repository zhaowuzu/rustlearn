use std::thread;
use chrono::Local;
use tokio::{self,runtime::Runtime,time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main(){
    let rt = Runtime::new().unwrap();
    rt.spawn_blocking(||{
        // 一个运行5秒的blocking thread
        // 删除rt时，该任务继续运行，直到自己终止
        // 会阻塞drop
       thread::sleep(std::time::Duration::from_secs(5));
        println!("blocking thread task over:{}",now())
    });

    // 进入runtime,并生成一个运行3秒的异步任务，
    // 删除rt时，该任务直接终止，因为这些异步线程不应该被阻塞
    let _guard = rt.enter();
    rt.spawn(async{
       time::sleep(time::Duration::from_secs(3)).await;
        println!("worker thread task over 1:{}",now()); // 这个不会被打印出来
    });
    // 进入runtime,并生成一个运行4秒的异步任务，
    // 删除rt时，该任务继续运行，直到自己终止
    // 虽不会阻塞drop,但是线程不会被杀死，会善终的,因为不属于tokio，已经属于系统了
    rt.spawn(async{
        thread::sleep(std::time::Duration::from_secs(4));
        println!("worker thread task over 2:{}",now());
    });

    // 先让所有任务运行起来
    std::thread::sleep(std::time::Duration::from_millis(3));

    // // 删除runtime句柄，将直接移除那个3秒的异步任务。
    // // 且阻塞5秒，直到所有自己已经阻塞的thread完成
    // drop(rt);
    // println!("runtime droped:{}",now());

    // 需要注意的是，强行关闭Runtime，可能会使得尚未完成的任务的资源泄露。因此，应小心使用强行关闭Runtime的操作
    // // 1 秒后强行关闭runtime
    // rt.shutdown_timeout(std::time::Duration::from_secs(1));
    // println!("runtime droped:{}",now());
    // 立即关闭runtime
    rt.shutdown_background();
    println!("runtime droped:{}",now());



}