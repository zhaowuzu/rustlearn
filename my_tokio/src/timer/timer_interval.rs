use chrono::Local;
use tokio::{self,runtime::Runtime,time::{self,Duration,Instant}};
use tokio::time::MissedTickBehavior;

/*
interval_at(): 接收一个Instant参数和一个Duration参数，Instant参数表示间隔计时器的开始计时点，Duration参数表示间隔的时长
interval(): 接收一个Duration参数，它在第一次被调用的时候立即开始计时
注意，这两个函数只是定义了间隔计时器的起始计时点和间隔的时长，要真正开始让它开始计时，还需要调用它的tick()方法生成一个Future任务，并调用await来执行并等待该任务的完成
*/

/*
interval_at()第一个参数定义的是计时器的开始时间，这样描述不准确，它表述的是最早都要等到这个时间点才开始计时。例如，定义计时器5秒之后开始计时，但在第一次tick()之前，先睡眠了10秒，那么该计时器将在10秒后才开始，但如果第一次tick之前只睡眠了3秒，那么还需再等待2秒该tick计时任务才会完成。
定义计时器时，要将其句柄(即计时器变量)声明为mut，因为每次tick时，都需要修改计时器内部的下一次计时起点。
不像其它语言中的间隔计时器，tokio的间隔计时器需要手动调用tick()方法来生成临时的异步任务。
删除计时器句柄可取消间隔计时器。
*/

fn now()-> String{
    Local::now().format("%F %T").to_string()
}


fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async{


        // 计时器的起始计时点，此时此刻之后的5秒后
        let start = Instant::now() + Duration::from_secs(5);
        let interval = Duration::from_secs(1);
        {
            let mut intv = time::interval_at(start,interval);
            println!("before:{}",now());
            // 该计时任务“阻塞”，直到5秒后被唤醒
            intv.tick().await;
            println!("task 1:{}",now());

            // 该计时任务“阻塞”，直到1秒后被唤醒
            intv.tick().await;
            println!("task 2:{}",now());

            // 该计时任务“阻塞”，直到1秒后被唤醒
            intv.tick().await;
            println!("task 3:{}",now());

            //before:2023-01-10 18:35:46
            // task 1:2023-01-10 18:35:51
            // task 2:2023-01-10 18:35:52
            // task 3:2023-01-10 18:35:53
        };
        {
            /*
            定义5秒后开始的计时器intv，该计时器内部有一个字段记录着下一次开始tick的时间点，其值为19:00:15
            睡眠10秒后，时间点到了19:00:20，此时第一次执行intv.tick()
            ，它将生成一个异步任务，执行器执行时发现此时此刻的时间点已经超过该计时器内部记录的值，于是该异步任务立即完成并进入就绪队列等待调度，同时修改计时器内部的值为19:00:16
            下一次执行tick的时候，此时此刻仍然是19:00:20，已经超过了该计时器内部的19:00:16，因此计时任务立即完成
            */
            // 这是tokio Interval在遇到计时延迟时的默认计时策略，叫做Burst
            let mut intv = time::interval_at(start,interval);

            println!("1>before1:{}",now());

            time::sleep(Duration::from_secs(10)).await;
            println!("1>before2:{}",now());

            // 该计时任务“阻塞”，直到5秒后被唤醒
            intv.tick().await;
            println!("1>task 1:{}",now());

            // 该计时任务“阻塞”，直到1秒后被唤醒
            intv.tick().await;
            println!("1>task 2:{}",now());

            //1>before1:2023-01-10 18:35:53
            // 1>before2:2023-01-10 18:36:03
            // 1>task 1:2023-01-10 18:36:03
            // 1>task 2:2023-01-10 18:36:03
        };
        {
            /*
            2.Delay策略，延迟性的计时策略，当出现延迟后，仍然按部就班地每隔指定的时长计时」。在内部，这种策略是在每次执行tick之后，都修改下一次计时起点为Instant::now() + Duration
。因此，这种策略下的任何相邻两次的tick，其中间间隔的时长都至少达到Duration。
            */
            let mut intv = time::interval_at(   Instant::now() + Duration::from_secs(5),
                                                Duration::from_secs(2),);
            intv.set_missed_tick_behavior(MissedTickBehavior::Delay);

            println!("2>before1:{}",now());

            time::sleep(Duration::from_secs(10)).await;
            println!("2>before2:{}",now());


            intv.tick().await;
            println!("2>task 1:{}",now());


            intv.tick().await;
            println!("2>task 2:{}",now());

            intv.tick().await;
            println!("2>task 3:{}",now());

            //2>before1:2023-01-10 18:36:03
            // 2>before2:2023-01-10 18:36:13
            // 2>task 1:2023-01-10 18:36:13
            // 2>task 2:2023-01-10 18:36:15
            // 2>task 3:2023-01-10 18:36:17
        };
        {
            /*
           「3.Skip策略，忽略型的计时策略，当出现延迟后，仍然所有已经被延迟的计时任务」。这种策略总是以定义计时器时的起点为基准，类似等差数量，每一次执行tick的时间点，一定符合Start + N * Duration
            */
            let mut intv = time::interval_at(Instant::now() + Duration::from_secs(5),
                                             Duration::from_secs(2),);
            intv.set_missed_tick_behavior(MissedTickBehavior::Skip);

            println!("3>before1:{}",now());

            time::sleep(Duration::from_secs(10)).await;
            println!("3>before2:{}",now());


            intv.tick().await;
            println!("3>task 1:{}",now());


            intv.tick().await;
            println!("3>task 2:{}",now());

            intv.tick().await;
            println!("3>task 3:{}",now());

            intv.tick().await;
            println!("3>task 4:{}",now());

            intv.tick().await;
            println!("3>task 5:{}",now());


            // 3>before1:2023-01-10 18:38:58
            // 3>before2:2023-01-10 18:39:08
            // 3>task 1:2023-01-10 18:39:08
            // 3>task 2:2023-01-10 18:39:09
            // 3>task 3:2023-01-10 18:39:11
            // 3>task 4:2023-01-10 18:39:13
            // 3>task 5:2023-01-10 18:39:15

        }

    });
}

/*
「1.Burst策略，冲刺型的计时策略，当出现延迟后，将尽量快地完成接下来的tick，直到某个tick赶上它正常的计时时间点」。
「2.Delay策略，延迟性的计时策略，当出现延迟后，仍然按部就班地每隔指定的时长计时」
「3.Skip策略，忽略型的计时策略，当出现延迟后，仍然所有已经被延迟的计时任务」


解释清楚了tokio::time::Interval
的三种计时策略。但在程序中，更大的可能是使用interval()
来定义间隔计时器，它等价于interval_at(Instant::now() + Duration)
，表示计时起点从现在开始的计时器。

此外，可以使用period()
方法获取计时器的间隔时长，使用missed_tick_behavior()
获取当前的计时策略。
*/