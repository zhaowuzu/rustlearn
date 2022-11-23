//use std::borrow::Borrow;
use std::fs;
use std::error::Error;
use std::env;

pub struct Config{
    pub query: String,
    pub filename:String,
    pub case_sensitive:bool,
}
impl Config{
    pub fn new (args:&[String]) -> Result<Config,&'static str> { // 字符串字面量的类型 都是静态生命周期
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();  // clone 的取舍当前先不用过多的考虑
        let filename = args[2].clone(); // clone 的取舍当前先不用过多的考虑

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // 用is_err()来检查结果是否错误，如果环境变量被设置了某个值，则返回假
        Ok(Config{query,filename,case_sensitive})
    }
}

// 执行逻辑
pub fn run(config:Config)-> Result<(),Box<dyn Error>>{ // dyn 表示动态的意思，因为返回的Error这个错误类型是一个trait，并不清楚具体的类型是什么，所以加上dyn关键字表示动态的意思。
    let contents = fs::read_to_string(config.filename)?;// ？运算符不同与pannic！可以将错误返回给函数的调用者来进行处理。取代expect，

    let results = if config.case_sensitive {
        search_case_insensitive(&config.query,&contents)
    }else{
        search(&config.query,&contents)
    };

    for line in results{
        println!("{}",line);
    }

    Ok(())
}

pub fn search<'a>(query:&str,contents:&'a str) ->Vec<&'a str>{ // 因为传入的参数有2个，所以要明确告知传出的结果依赖哪个传入的生命周期；
    let mut results = Vec::new();
    for line in contents.lines() {
        // do someting with line
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query:&str,contents:&'a str) ->Vec<&'a str>{ // 因为传入的参数有2个，所以要明确告知传出的结果依赖哪个传入的生命周期；
   let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        // do someting with line
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust
saft,fast,productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["saft,fast,productive."],
            search(query,contents)
        );
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
saft,fast,productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:","Trust me."],
            search_case_insensitive(query,contents)
        );
    }
}