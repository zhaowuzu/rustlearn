/*
注：std::time
也提供了sleep()，但它会阻塞整个线程，而tokio::time
中的sleep()则只是让它所在的任务放弃CPU并进入调度队列等待被唤醒，它不会阻塞任何线程，它所在的线程仍然可被用来执行其它异步任务。因此，在tokio runtime中，应当使用tokio::time
中的sleep()
*/

use tokio::runtime::Runtime;
use chrono::Local;


fn main(){
    let rt = Runtime::new().unwrap();
    // block_on 会阻塞当前线程，直到指定的**异步任务树（可能又全部子任务）** 全部完成
    // 注：block_on是等待异步任务完成，而不是等待runtime中的所有任务都完成，后面介绍blocking thread时会再次说明block_on的阻塞问题。
    rt.block_on(async{ // block_on 需要传入一个Future作为参数
       println!("before sleep:{}",Local::now().format("%F %T.%3f"));
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; // await可以将一定义好的异步任务立即加入到runtime的任务队列中等待调度执行，于此同时await会等待该异步任务完成才返回
        println!("before sleep:{}",Local::now().format("%F %T.%3f"));
        // await的第二个使用
        let task = tokio::time::sleep(tokio::time::Duration::from_secs(2));
        //不会执行
        //...

        // 开始执行
        task.await;
        // 等待阻塞完毕后开始执行
        println!("before sleep:{}",Local::now().format("%F %T.%3f"));
    });

    // block_on的返回值
    let res = rt.block_on(async{
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        3
    });
    println!("{}",res);
}