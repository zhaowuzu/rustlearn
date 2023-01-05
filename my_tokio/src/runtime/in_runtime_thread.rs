use tokio::{self,runtime::Runtime,time};
use std::thread;
use chrono::Local;

fn now() -> String{
    Local::now().format("%F %T").to_string()
}

fn main(){
    let rt = Runtime::new().unwrap();
    // rt.block_on(async{
    //    // 在runtime中，让整个线程进入睡眠，注意不是tokio::time::sleep()
    //     // 脱离了tokio的控制，在一定程度上破坏了tokio的调度系统
    //     std::thread::sleep(std::time::Duration::from_secs(10));
    // });

    // 创建一个blocking thread.可立即执行（由操作系统调度系统决定何时执行）
    // 注意，不阻塞当前线程
    let task = rt.spawn_blocking(||{
       println!("in task:{}",now());
        // 注意，是线程的睡眠，不是tokio的睡眠，因此会阻塞整个线程
        thread::sleep(std::time::Duration::from_secs(10))
    });

    // 小睡1毫秒，让上面的blocking thread先运行起来
    thread::sleep(std::time::Duration::from_secs(1));
    println!("not blocking:{}",now());

    // 可在runtime内等待blocking_thread的完成
    rt.block_on(async{
        task.await.unwrap();
        println!("after blocking task:{}",now());
    });


    println!("看看会不会直接返回:{}",now());
    rt.block_on(async{
        // 生成一个blocking thread 和一个独立的thread
        // block on 不会阻塞等待两个线程终止，因此block_on在这里会立即返回
        let X = rt.spawn_blocking(||std::thread::sleep(std::time::Duration::from_secs(10)));
        println!("内部1:{}",now());
        let Y = thread::spawn(||std::time::Duration::from_secs(10));
        println!("内部2:{}",now());
    });
    println!("看看会不会直接返回，查看:{}",now());

    // 这里不会直接结束进程，而是等待了上一步中存在的X结束，才正式结束进程。不会等待Y。

    /*
    注意，这种删除runtime句柄的方式只会立即关闭未被阻塞的worker thread，那些已经运行起来的blocking thread以及已经阻塞整个线程的worker thread仍然会执行。
    但是，删除runtime又要等待runtime中的所有异步和非异步任务(会阻塞线程的任务)都完成，因此删除操作会阻塞当前线程。
    */

    drop(rt);// 关闭Runtime 会被阻塞，等待X完成
    println!("哦，被主动关闭了:{}",now());

}