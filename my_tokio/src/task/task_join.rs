/*
join!:必须等待所有任务完成
try_join!:要么等待所有异步任务正常完成，要么等待第一个返回Result Err的任务出现
*/

use std::thread;
use chrono::Local;
use tokio::{self,runtime::Runtime,time};

fn now()-> String{
    Local::now().format("%F %T").to_string()
}

async fn do_one(){
    println!("doing one:{}",now());
    time::sleep(time::Duration::from_secs(2)).await;
    println!("do one done:{}",now());
}

async fn do_two(){
    println!("doing two:{}",now());
    time::sleep(time::Duration::from_secs(1)).await;
    println!("do two done:{}",now());
}

async fn do_one_try() -> Result<(), &'static str>{
    println!("doing one try:{}",now());
    time::sleep(time::Duration::from_secs(2)).await;
    println!("do one done try:{}",now());
    Ok(())
}

async fn do_two_try() -> Result<(), &'static str>{
    println!("doing two try:{}",now());
    time::sleep(time::Duration::from_secs(1)).await;
    println!("do two done try:{}",now());
    Ok(())
}

fn main(){
    let rt = Runtime::new().unwrap();
    rt.spawn(async{
        let res = tokio::try_join!(do_one_try(),do_two_try());
        match res {
            Ok((first, second)) => {
                // do something with the values
                println!("all try_join done:{},{:?},{:?}",now(),first,second)
            }
            Err(err) => {
                println!("processing failed; error = {}", err);
            }
        }
    });

    rt.block_on(async{
        tokio::join!(do_one(),do_two());
        println!("all join done:{}",now())
    });

   thread::sleep(time::Duration::from_secs(3));
    println!("end:{}",now())
}