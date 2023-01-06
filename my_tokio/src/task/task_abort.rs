use tokio::{self,runtime::Runtime,time};
use tokio::task::JoinError;

fn main(){
  let rt = Runtime::new().unwrap();
    rt.block_on(async{
        let task = tokio::task::spawn(async {
            println!("执行了");
           time::sleep(time::Duration::from_secs(10)).await;
            println!("执行到最后了");
        });

        // 让上面的异步任务跑起来
        time::sleep(time::Duration::from_millis(1)).await;
        task.abort(); // 取消任务
        // 取消任务之后，可以取消JoinError
        let abort_err:JoinError = task.await.unwrap_err();
        println!("{}",abort_err.is_cancelled());
    });
}