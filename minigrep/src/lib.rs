use std::fs;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub filename:String,
}
impl Config{
    pub fn new (args:&[String]) -> Result<Config,&'static str> { // 字符串字面量的类型 都是静态生命周期
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();  // clone 的取舍当前先不用过多的考虑
        let filename = args[2].clone(); // clone 的取舍当前先不用过多的考虑

        Ok(Config{query,filename})
    }
}

// 执行逻辑
pub fn run(config:Config)-> Result<(),Box<dyn Error>>{ // dyn 表示动态的意思，因为返回的Error这个错误类型是一个trait，并不清楚具体的类型是什么，所以加上dyn关键字表示动态的意思。
    let contents = fs::read_to_string(config.filename)?;// ？运算符不同与pannic！可以将错误返回给函数的调用者来进行处理。取代expect，

    println!("With text:\n{}",contents);

    Ok(())
}