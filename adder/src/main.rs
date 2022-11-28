// 存放二进制的
use adder;
use add_one;

fn main(){
    let val = adder::add(2,2);
    println!("{}",val);
    let guess = add_one::add(1,1);
}