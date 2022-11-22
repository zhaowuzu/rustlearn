use std::env;
// use std::env::args_os;   //会得到OsString的迭代器

fn main() {
    let args :Vec<String> = env::args().collect();
    println!("{:?}",args);
}
