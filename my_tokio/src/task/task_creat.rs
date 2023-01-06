use std::thread;
use chrono::Local;
use tokio::{self,task,runtime::Runtime,time,runtime::Handle};

fn now() -> String{
    Local::now().format("%F %T").to_string()
}

#[tokio::main] // 默认创建了一个多线程的runtime
async fn main(){
    // 对main 修饰#[tokio::main] 就不在需要下面的这2行了
    // let rt = Runtime::new().unwrap();
    // let _guard = rt.enter();

    // // # spawn 的使用
    // // 向runtime中添加新异步任务
    // // 异步任务不会产生阻塞效果的，所以如果不给其充足的执行时间，就结束了runtime，那么这个异步的任务也会直接会被搞死
    // task::spawn(async{
    //    time::sleep(time::Duration::from_secs(3)).await;
    //     println!("task over:{}",now())
    // });
    // thread::sleep(std::time::Duration::from_secs(4));
    //
    // // # spawn_blocking 的使用
    // // 生成一个blocking thread并执行指定的任务
    // spawn_blocking().await;
    // println!("spawn_blocking end:{}",now());
    //
    // // # block_in_place 的使用
    // // 在某个worker thread中执行同步任务，但是会将同线程中的其它异步任务转移走，使得异步任务不会被同步任务饥饿(因为被移入到其它线程了吗)
    // // block_in_place得到的仍是一个普通的异步任务
    // task::block_in_place(move || { // 把runtime移入至闭包中，但是应该只是引用了一个rt的引用
    //     Handle::current().block_on(async move {
    //         // do something async
    //         time::sleep(time::Duration::from_secs(3)).await;
    //         println!("block_in_place over:{}",now())
    //     });
    // });
    //
    // // # yield_now 的使用
    // // 立即放弃CPU，将线程交还给调度器，自己则进入就绪队列等待下一轮的调度
    // task::yield_now().await;
    // println!("yield_now end:{}",now());

    // # unconstrained 的使用
    // 将指定的异步任务声明未不受限的异步任务，它将不受tokio的协作式调度，它将一直霸占当前线程直到任务完成，不会受到tokio调度器的管理
    // 因此，unconstrained()创建的异步任务将会使得同线程的其它异步任务被饥饿。如果确实有这样的需求，建议使用block_in_place()或spawn_blocking()
    task::spawn( async {  // 这个只是做个对比
        time::sleep(time::Duration::from_secs(1)).await;
        println!("unconstrained  comparison over:{}",now())
    });
    task::unconstrained(async {
        time::sleep(std::time::Duration::from_secs(1)).await; // 这个东西没有办法演示这个东东，本身就不会阻塞
        println!("unconstrained self over:{}",now())
    }).await;
    thread::sleep(std::time::Duration::from_secs(2));

    // # spawn_local 的使用
    // 生成一个在当前线程内运行，一定不会被偷到其它线程中运行的异步任务

    println!("end:{}",now())
}

async fn spawn_blocking(){
    let rt1 = Runtime::new().unwrap();
    let _guard = rt1.enter();

    let join = task::spawn_blocking(||{
        // todo do some cpmpute-heavy work or call synchronous code
        "blocking completed"
    });
    let result = join.await.unwrap();
    assert_eq!(result,"blocking completed");

    // thread 'main' panicked at 'Cannot drop a runtime in a context where blocking is not allowed. This happens when a runtime is dropped from within an asynchronous context.
    // 上面的错误就是没有办法主动关掉的意思，需要我们人为出手关闭掉它
    rt1.shutdown_background();
}