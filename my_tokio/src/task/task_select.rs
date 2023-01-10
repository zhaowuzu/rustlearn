use tokio::{self, runtime::Runtime, time::{self, Duration}};

async fn sleep(n:u64)-> u64 {
    time::sleep(Duration::from_secs(n)).await;
    println!("{}",n);
    n
}

fn main(){
    let rt = Runtime::new().unwrap();
    rt.block_on(async{
       tokio::select! {
           v = sleep(5) => println!("sleep 5 secs,branch 1 done:{}",v), // 不在等待这个异步任务完成,生命周期结束了
           v = sleep(3) => println!("sleep 3 secs,branch 2 done:{}",v), // 结束select
       } ;

        println!("select! done");
    });

    rt.block_on(async{
        let mut count = 0u8;
        loop{
            tokio::select! {
           // 如果取消biased,挑选的任务顺序将随机，可能会导致分支中的断言失败
           biased;
           _ = async{println!("1号");},if count < 1 => {count += 1;assert_eq!(count,1);} // 如果if不满足，则后续的{}不在执行，async也不会被执行
           _ = async{println!("2号");},if count < 2 => {count += 1;assert_eq!(count,2);}
           _ = async{println!("3号");},if count < 3 => {count += 1;assert_eq!(count,3);}
           _ = async{println!("4号");},if count < 4 => {count += 1;assert_eq!(count,4);}
           else => {break;}
            };
        };
        println!("count end:{}",count)
    });

}