use std::{env, process};
// use std::env::args_os;   //会得到OsString的迭代器
use minigrep::Config; // 把lib里面的结构引出来

fn main() {
    let args :Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{ // 闭包的参数被写在两条竖线之间
       println!("Problem parsing arguments:{}",err);
        process::exit(1);
    });
    // let config  = Config::new(&args);

    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);

    if let Err(e)= minigrep::run(config) { // 考虑和unwrap_or_else 的不同，我们不需要取出正确的Ok值,所以这个只时找到Err就可以了。
        println!("Application error:{}",e);
        process::exit(1);
    }
}