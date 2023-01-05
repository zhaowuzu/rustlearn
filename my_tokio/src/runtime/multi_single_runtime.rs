use tokio;
use std::thread;

fn main(){
    let t1 = thread::spawn(||{
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async{

        })
    });

    let t2 = thread::spawn(||{
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async{

        })
    });

    t1.join().unwrap();
    t2.join().unwrap();
}