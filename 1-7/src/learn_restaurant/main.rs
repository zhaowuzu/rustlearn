use rand::Rng;
use std::{cmp::Ordering,io};
use std::io::{self,Write}; // 嵌套引入
use std::collections::*; // 通配符

fn main(){
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("hello world {}",secret_number);
}