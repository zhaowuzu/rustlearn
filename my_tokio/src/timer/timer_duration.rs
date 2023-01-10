use tokio::{time::Duration};

fn main(){
    // 创建
    // 5秒30纳秒  5.00000003s
    println!("C:{:?}",Duration::new(5,30));
    println!("C:{:?}",Duration::from_secs(3)); // 3秒时长
    println!("C:{:?}",Duration::from_millis(300));// 300毫秒时长
    println!("C:{:?}",Duration::from_micros(300));// 300微秒时长
    println!("C:{:?}",Duration::from_nanos(300));// 300纳秒时长
    println!("C:{:?}",Duration::from_secs_f32(2.3));// 2.3秒时长
    println!("C:{:?}",Duration::from_secs_f64(2.3));// 2.3秒时长

    // 转换
    let dur = Duration::from_secs_f32(2.3);
    println!("T:{:?}",dur.as_secs()); // 转换为秒的表示方式，2
    println!("T:{:?}",dur.as_millis()); // 转换为毫秒表示方式，2300
    println!("T:{:?}",dur.as_micros()); // 转换为微秒表示方式，2_300_000
    println!("T:{:?}",dur.as_nanos()); // 转换为纳秒表示方式，2_300_000_000
    println!("T:{:?}",dur.as_secs_f32()); // 小数秒表示方式，2.3
    println!("T:{:?}",dur.as_secs_f64()); // 小数秒表示方式，2.3
    println!("T:{:?}",dur.subsec_millis()); // 小数部分转换为毫秒精度的表示方式，300
    println!("T:{:?}",dur.subsec_micros()); // 小数部分转换为微秒精度的表示方式，300_000
    println!("T:{:?}",dur.subsec_nanos()); // 小数部分转换为纳秒精度的表示方式，300_000_000

    // 时间的加减乘除
    println!("U:{:?}",dur.checked_add(Duration::from_secs(2))); // 时长的加法运算，超出Duration范围时返回None
    println!("U:{:?}",dur.checked_sub(Duration::from_secs(2))); // 时长的减法运算，超出Duration范围时返回None
    println!("U:{:?}",dur.checked_mul(2)); // 时长的乘法运算，超出Duration范围时返回None
    println!("U:{:?}",dur.checked_div(2)); // 时长的除法运算，超出Duration范围时(即分母为0)返回None
    println!("U:{:?}",dur.saturating_add(Duration::from_secs(2))); //饱和式的加法运算，超出范围时返回Duration支持的最大时长
    println!("U:{:?}",dur.saturating_mul(2)); // 饱和式的乘法运算，超出范围时返回Duration支持的最大时长
    println!("U:{:?}",dur.saturating_sub(Duration::from_secs(2))); // 饱和式的减法运算，超出范围时返回0时长
    println!("U:{:?}",dur.mul_f32(2.1)); // 时长乘以小数，得到的结果如果超出范围或无效，则panic
    println!("U:{:?}",dur.mul_f64(2.1)); // 时长乘以小数，得到的结果如果超出范围或无效，则panic
    println!("U:{:?}",dur.div_f32(2.1)); // 时长除以小数，得到的结果如果超出范围或无效，则panic
    println!("U:{:?}",dur.div_f64(2.1)); // 时长除以小数，得到的结果如果超出范围或无效，则panic
}