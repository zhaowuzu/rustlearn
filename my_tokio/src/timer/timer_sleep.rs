use tokio::{self,runtime::Runtime,time};
use chrono::Local;

#[allow(dead_code)]
fn now()-> String{
    Local::now().format("%F %T").to_string()
}

/*
注意，tokio的sleep的睡眠精度是毫秒，因此无法保证也不应睡眠更低精度的时间。例如不要睡眠100微秒或100纳秒，这时无法保证睡眠的时长。
*/

fn main(){
    let rt = Runtime::new().unwrap();
    rt.block_on(async{
        println!("1:{}",now());
       // 睡2秒
        time::sleep(time::Duration::from_secs(2)).await;
        println!("2:{}",now());

        // 一直睡眠，睡到2秒后醒来
        time::sleep_until(time::Instant::now()+time::Duration::from_secs(2)).await;
        println!("3:{}",now());

        // 太精细的休眠会有差距，尽量不要这样使用
        let start = time::Instant::now();
        time::sleep(time::Duration::from_micros(10)).await;
        println!("sleep {}", time::Instant::now().duration_since(start).as_nanos());

        // deadline(): 返回Instant，表示该睡眠任务的睡眠终点
        // is_elapsed(): 可判断此时此刻是否已经超过了该sleep任务的睡眠终点
        // reset()：可用于重置睡眠任务。如果睡眠任务未完成，则直接修改睡眠终点，如果睡眠任务已经完成，则再次创建睡眠任务，等待新的终点
        println!("start:{}",now());
        let slp=time::sleep(time::Duration::from_secs(1));
        tokio::pin!(slp);
        slp.as_mut().reset(time::Instant::now()+time::Duration::from_secs(2));
        // 注意调用slp.as_mut().await,而不是slp.await，后置move消费掉slp
        slp.as_mut().await;
        println!("end:{}",now());
        // 重置已完成的睡眠
        slp.as_mut().reset(time::Instant::now()+time::Duration::from_secs(2));
        slp.await;
        println!("end:{}",now());

    });
}