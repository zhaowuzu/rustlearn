use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main(){
   // let handle =  thread::spawn(||{
   //      for i in 1..10 {
   //          println!("hi number {} from the spawned thread!",i);
   //          thread::sleep(Duration::from_millis(1));
   //      }
   //  });
   //
   //  //handle.join().unwrap(); // join 类似wait，等待线程执行完毕  执行完毕后，再会下面循环执行
   //
   //  for i in 1..5 {
   //      println!("hi number {} from the main thread!",i);
   //      thread::sleep(Duration::from_millis(1));
   //  };
   //
   //  //handle.join().unwrap();

    // thread::spawn , join ,move 的使用
    let v = vec![1,2,3];

    let handle = thread::spawn(move ||{ // 强制闭包获取它所需值的所有权，而不仅仅是基于Rust的推导获取值的借用。
        println!("Here's a vector: {:?}",v);
    });

    // drop(v); // 情况不妙，直接丢弃v

    handle.join().unwrap(); // join 做阻塞。


    // 使用消息传递在线程间转移数据
    // mpsc=multiple producer,single consumer,多个生产者，单个消费者 m:1
    // 带有模式的let进行拆解
    let (tx,rx) = mpsc::channel(); // 创建一个通道
    thread::spawn(move||{ // 用move 获取tx的所有权
        let val = String::from("hi");
        // move occurs because `val` has type `String`, which does not implement the `Copy` trait
        tx.send(val).unwrap();//调用了unwrap来触发pannic  send函数会获取参数的所有权，并在参数传递时将所有权转移给接收者。
        // value moved here
        // println!("val is {}",val); // 这个里面是报错。
    });
    //let received = rx.recv().unwrap();// recv会阻塞线程，直到有值被传入。try_recv不会阻塞，会立即返回，存在消息就返回OK，否则返回Err变体。
    //println!("Got:{}",received);
    /*
    Err:receiving on an empty channel
    Err:receiving on an empty channel
    Got:hi
    */
    loop{
        match  rx.try_recv(){
            Ok(val)=> {println!("Got:{}",val);break;},
            Err(e) => println!("Err:{}",e)
        }
    }

    // 发生多个值并观察接收者的等待过程
    let (tx1,rx1) = mpsc::channel();

    let tx2 = mpsc::Sender::clone(&tx1);
    thread::spawn(move||{
        let vals = vec!{
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        };
        for val in vals{
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move||{
        let vals = vec!{
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        };
        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx1{ // 不再显示调用recv,rx1视为迭代器
        println!("Got:{}",received) // 输出的顺序不确定
    }
}
