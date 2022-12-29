use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct ThreadPool{
    workers:Vec<Worker>,
    sender:mpsc::Sender<Message>, // 发生端
}

type Job = Box<dyn FnOnce() + Send + 'static>; // Job 存放一个trait，而且还做了相关的限定

impl ThreadPool{
    /// 创建线程池
    ///
    /// 线程池中线程的数量
    ///
    /// # Panics
    ///
    /// 'new' 函数会在size为0时触发panic.
    pub fn new(size:usize)->ThreadPool {
        assert!(size>0);

        let (sender,receiver) = mpsc::channel(); // 1个发送者，N个消费者

        let receiver = Arc::new(Mutex::new(receiver)); // 所以使用Arc来共享资源

        let mut  workers = Vec::with_capacity(size);// with_capacity会为动态数组预分配出指定空间

        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }

        ThreadPool{
            workers,
            sender
        }


    }

    pub fn execute<F>(&self,f:F)
    where
        F:FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}
impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // 向所有的工作者发送关闭通知
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in & mut self.workers{
            println!("Shutting down worker {}",worker.id);
            // 使用take把移植过来，并在原来位置放置一个None
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,// 利用Option的take方法将Some变体的值移出来，而在原来的位置留下None变体
}

impl Worker {
    fn new(id :usize,receiver:Arc::<Mutex<mpsc::Receiver<Message>>>)->Worker {
        // TODO thread::spawn 创建后会直接执行？？
        let thread = thread::spawn(move ||{ // move 将引用的变量的所有权转移至闭包内，通常用于使闭包的生命周期大于所捕获的变量的原生命周期（例如将闭包返回或移至其他线程）
            loop{
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.",id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.",id);
                        break
                    }
                }
            }

            // // 一种错误的用法
            // // 原因nutex没有存在unlock方法，依赖MutexGuard<T>的生命周期。
            // // while表达式内的值会把整个代码视作自己的作用域，我们在调用job()的过程中仍然持有着receiver的锁
            // while let Ok(job) = receiver.lock().unwrap().recv(){
            //     println!("Worker {} got a job; executing.",id);
            //     job();
            // }

        });

        Worker{
            id,
            thread:Some(thread),// 用Some包一下子
        }
    }
}

enum Message{
    NewJob(Job), // 这个其实使给变体NewJob关联一个Job类型的值
    Terminate,
}