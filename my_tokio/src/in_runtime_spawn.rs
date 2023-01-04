/// spawn:向runtime中添加新的异步任务

use std::thread;
use chrono::Local;
use tokio::{self,runtime::Runtime,time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

// 在runtime外部定义一个异步任务，且该函数返回值不是Future类型
fn async_task(){
    println!("create an async task:{}",now());
    tokio::spawn(async{
       time::sleep(time::Duration::from_secs(10)).await;
        println!("async task over:{}",now());
    });
}

fn async_task_tokio(rt: &Runtime){
    println!("create an async task2:{}",now());
    rt.spawn(async{
        time::sleep(time::Duration::from_secs(10)).await;
        println!("async task over2:{}",now());
    });
}

fn main() {
    let rt1 = Runtime::new().unwrap();
    rt1.block_on(async{
        // 调用函数，该函数内创建了一个异步任务，将在当前runtime内执行
        async_task();
        async_task_tokio(&rt1);
    });

    thread::sleep(std::time::Duration::from_secs(15));
}