use std::thread;
use chrono::Local;
use tokio::{self,runtime::Runtime,time};

fn now()->String{
    Local::now().format("%F %T").to_string()
}

fn main(){
    let rt = Runtime::new().unwrap();
    // 创建一个LocalSet队列
    let local_tasks = tokio::task::LocalSet::new();

    // 向本地任务队列中添加新的异步任务，但现状不会执行
    local_tasks.spawn_local(async{
        println!("local task1");
        time::sleep(time::Duration::from_secs(5)).await;
        println!("local task1 done");
    });

    local_tasks.spawn_local(async{
        println!("local task2");
        time::sleep(time::Duration::from_secs(5)).await;
        println!("local task2 done");
    });

    println!("before local tasks running:{}",now());

    // # 第一种方法，使用await
    // rt.block_on(async{
    //    // 开始执行本地任务队列中的所有异步任务，并等待它们全部完成
    //     local_tasks.await;
    // });

    // # 第二种方法LocalSet::block_on进入LocalSet上下文,
    // 注意：block_on参数中的future需要做await，经过测试，会发现block_on之后被阻塞future的时间，执行完就结束了block_on，不在关心队列里面其它的异步任务，所以这个方法调用不是很灵活和优秀
    // 需要注意的是，调用LocalSet::block_on()和LocalSet::run_until(时均需指定一个异步任务(Future)作为其参数，它们都会立即开始执行该异步任务以及本地任务队列中已存在的任务，
    // 但是这两个函数均只等待其参数对应的异步任务执行完成就返回。这意味着，它们返回的时候，可能还有正在执行中的本地异步任务，它们会继续保留在本地任务队列中。
    // 当再次进入LocalSet上下文或await LocalSet的时候，它们会等待调度并运行。
    // local_tasks.block_on(&rt,async{
    //     local_tasks.spawn_local(async{
    //         println!("local task3");
    //         time::sleep(time::Duration::from_secs(1)).await;
    //         println!("local task3 done");
    //     }).await.unwrap()
    // });
    // // 再次进入LocalSet时，会再次执行LocalSet队列的任务
    // rt.block_on(async{
    //     // 开始执行本地任务队列中的所有异步任务，并等待它们全部完成
    //     local_tasks.await;
    // });

    // # 第三种方法LocalSet::run_until进入LocalSet上下文,
    // 和block_on()类似，区别仅在于它只能在Runtime::block_on()内或[tokio::main]注解的main函数内部被调用。
    // 需要注意的和block_on一样，参考一下。正常情况下确实不推荐
    rt.block_on(async{
        local_tasks.run_until(async{
            println!("loacl task3");
            tokio::task::yield_now().await;
            println!("local task3 done:{}",now());
        }).await;
    });

    println!("end:{}",now())
}

