use tokio::{self, time::Duration, time::Instant};

#[tokio::main]
async fn main(){
    // 创建代表此时此课的时间点
    let now = Instant::now();

    // Instant 加一个Duration,得到另一个Instant
    let next_3_sec = now + Duration::from_secs(3);
    // Instant之间的大小比较
    println!("{}",now < next_3_sec);

    // Instant减duration,得到另一个Instant
    let new_instant = next_3_sec-Duration::from_secs(2);

    // Instant减另一个Instant，得到Duration
    // 注意，Duration有它的有效范围，因此必须是大的Instant减小Instant，反之pannic
    let duration = next_3_sec - new_instant;
    println!("{:?}",duration);
}
/*
此外tokio::time::Instant
还有以下几个方法：

from_std(): 将std::time::Instant
转换为tokio::time::Instant
into_std(): 将tokio::time::Instant
转换为std::time::Instant
elapsed(): 指定的时间点实例，距离此时此刻的时间点，已经过去了多久(返回Duration)
duration_since(): 两个Instant实例之间相差的时长，要求B.duration_since(A)
中的B必须晚于A，否则panic
checked_duration_since(): 两个时间点之间的时长差，如果计算返回的Duration无效，则返回None
saturating_duration_since(): 两个时间点之间的时长差，如果计算返回的Duration无效，则返回0时长的Duration实例
checked_add(): 为时间点加上某个时长，如果加上时长后是无效的Instant，则返回None
checked_sub(): 为时间点减去某个时长，如果减去时长后是无效的Instant，则返回None
tokio顶层也提供了一个tokio::resume()
方法，功能类似于tokio::time::from_std()
，都是将std::time::Instant::now()
保存为tokio::time::Instant
。不同的是，后者用于创建tokio time Instant时间点，而resume()
是让tokio的Instant的计时系统与系统的计时系统进行一次同步更新。
*/