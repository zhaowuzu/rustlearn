/*
非阻塞的enter()
1. block_on()进入runtime时，会阻塞当前线程
2. enter()进入runtime时，不会阻塞当前线程，它会返回一个EnterGuard

你需要知道：
EnterGuard没有其它作用，它仅仅只是声明从它开始的所有异步任务都将在runtime上下文中执行，直到删除该EnterGuard。
删除EnterGuard并不会删除runtime，只是释放之前的runtime上下文声明。因此，删除EnterGuard之后，可以声明另一个EnterGuard，这可以再次进入runtime的上下文环境。
*/

use tokio::{self,runtime::Runtime,time};
use chrono::Local;
use std::thread;

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main(){
    let rt = Runtime::new().unwrap();

    // 进入runtime,但不阻塞当前线程
    let guard1 = rt.enter();  // A:从该行以下的所以异步任务都会在在runtime里面执行

    // 生成的异步任务将放入当前的runtime上下文中执行
    tokio::spawn(async{
       time::sleep(time::Duration::from_secs(5)).await;
        println!("task1 sleep over:{}",now());
    });

    // 释放runtime上下文，这并不会删除runtime
    drop(guard1); // 到这就取消了A处的限制

    // // 这里的异步任务，缺少runtime的支持，会包pannic:must be called from the context of a Tokio 1.x runtime'
    // tokio::spawn(async{
    //     time::sleep(time::Duration::from_secs(5)).await;
    //     println!("task1 sleep over:{}",now());
    // });

    // 可以再次进入runtime
    let guard2 = rt.enter();
    tokio::spawn(async{
       time::sleep(time::Duration::from_secs(4)).await;
        println!("task2 sleep over:{}",now());
    });

    drop(guard2);

    // 阻塞当前线程，等待异步任务完成
    thread::sleep(std::time::Duration::from_secs(10));
}