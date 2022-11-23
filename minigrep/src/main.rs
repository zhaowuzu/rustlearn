use std::{env, process};
// use std::env::args_os;   //会得到OsString的迭代器
use minigrep::Config; // 把lib里面的结构引出来

// 运行 cargo run to poem.txt > output.txt
// 运行 cargo run  > output.txt

fn main() {
    let args :Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{ // 闭包的参数被写在两条竖线之间
       eprintln!("Problem parsing arguments:{}",err); // 打印到标准输出里面，不会打印到重定向的文件里面
        process::exit(1);
    });
    // let config  = Config::new(&args);

   // println!("Searching for {}",config.query);
  //  println!("In file {}",config.filename);

    if let Err(e)= minigrep::run(config) { // 考虑和unwrap_or_else 的不同，我们不需要取出正确的Ok值,所以这个只时找到Err就可以了。
        eprintln!("Application error:{}",e);
        process::exit(1);
    }
}