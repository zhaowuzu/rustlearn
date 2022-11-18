use std::fmt::Display;
use std::fmt::Debug;

// 使用trait的时候记得 “孤儿规则”

pub trait Summary{
    fn summarize_author(&self) -> String; // 常规定义
    fn summarize(&self) -> String { // 带默认值的定义
        // 带默认值的
        String::from("Read more ...")
    }
}

pub struct NewsArticle{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

impl Summary for NewsArticle{
    fn summarize_author(&self) -> String {
        format!("{}:{}",self.headline,self.content)
    }
}

// 覆盖标准库中的trait的方法
// impl ToString for NewsArticle{
//     fn to_string(&self)-> String{
//         String::from("self.location")
//     }
// }

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}

// implement 简写为impl
impl Summary for Tweet{
    fn summarize_author(&self) -> String {
        format!("{}:{}",self.username,self.content)
    }
    fn summarize(&self) -> String { // 默认的是可以被复写的
        format!("{}:{}",self.username,self.content)
    }
}


// 使用trait约束有条件地实现方法
#[derive(Debug)]
pub struct Pair<T>{
    x:T, // 私有的，所以在模块外部就不能通过 Pair{} 这样直接进行创建了
    y:T,
}

impl <T> Pair<T> {
    pub fn new(x:T,y:T) -> Self{
        Self{
            x,
            y,
        }
    }
}
impl<T:Display+PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("The largest member is x={}",self.x);
        }else{
            println!("The largest member is y={}",self.y);
        }
    }
}