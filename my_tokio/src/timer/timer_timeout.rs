use chrono::Local;
use tokio::{self,runtime::Runtime,time};

/*
tokio::time::timeout()或tokio::time::timeout_at()
可设置一个异步任务的完成超时时间，前者接收一个Duration和一个Future作为参数，后者接收一个Instant和一个Future作为参数。这两个函数封装异步任务之后，返回time::Timeout，它也是一个Future。
*/

/*
得到time::Timeout实例res后，可以通过res.get_ref()
或者res.get_mut()
获得Timeout所封装的Future的可变和不可变引用，使用res.into_inner()
获得所封装的Future，这会消费掉该Future。

如果要取消Timeout的计时等待，直接删除掉Timeout实例即可
*/
#[allow(dead_code)]
fn now()-> String{
    Local::now().format("%F %T").to_string()
}

fn main(){
    let rt = Runtime::new().unwrap();
    rt.block_on(async{
       let mut res = time::timeout(time::Duration::from_secs(5), async{
           println!("sleeping:{}",now());
           time::sleep(time::Duration::from_secs(3)).await;
           33
       }) ;
        // let aa =  res.get_ref();
        // //let mut aa =  res.get_mut();
        // // let mut aa =  res.into_inner(); // 会消费掉Future
        // rt.block_on(*aa);

        match res.await {
            Ok(data) => println!("get the res '{}':{}",data,now()),
            Err(_) => println!("task timeout:{}",now()),
        };
    });
}