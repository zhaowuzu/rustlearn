use tokio::{self,runtime::Runtime,time};

fn main(){
    let rt = Runtime::new().unwrap();
    let handle = rt.handle(); // 是runtime的一个引用
    handle.spawn(async{});
    handle.spawn_blocking(||{});
    handle.block_on(async{});
    handle.enter();
    drop(handle);
}