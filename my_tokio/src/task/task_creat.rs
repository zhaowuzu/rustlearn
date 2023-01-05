use std::thread;
use chrono::Local;
use tokio::{self,task,runtime::Runtime,time};

fn now() -> String{
    Local::now().format("%F %T").to_string()
}

fn main(){
    let rt = Runtime::new().unwrap();
    let _guard = rt.enter();

    task::spawn(async{
       time::sleep(time::Duration::from_secs(3)).await;
        println!("task over:{}",now())
    });

    thread::sleep(std::time::Duration::from_secs(4));

    drop(rt);

    spawn_blocking().await;
}

async fn spawn_blocking(){
    let rt = Runtime::new().unwrap();
    let _guard = rt.enter();

    let join = task::spawn_blocking(||{
        // todo do some cpmpute-heavy work or call synchronous code
        "blocking completed"
    });
    let result = join.await.unwrap();
    assert_eq!(result,"blocking completed1")
}