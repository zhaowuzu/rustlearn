use std::fmt::Arguments;
use std::io::Result;

pub fn demo(){
    // # 使用类型别名创建同义类型
    let x:i32 = 5;
    let y:Kilometers = 5;
    println!("x + y = {}",x+y); // 不同类型可以加，其实最内部是一样的类型

    let f:Thunk = Box::new(|| println!("hi")); // 这样使用起来就特别简单了

    // # 用不返回的Never类型
    //panic() // 相当于阻塞了，永远不会结束
    // 在系统中使用
    // let guess :u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue, // 其实这个就是一个 ！ 空类型
    // };
    // 下面这样写就是错的，是不允许返回两种类型的
    // let guess :u32 = match guess.trim().parse() {
    //     Ok(_) => 5,
    //     Err(_) => "Hello", // 其实这个就是一个 ！ 空类型
    // };
    // loop 也是以 ！作为返回类型的表达式
    print!("forever");
    loop {
        print!("and ever");
    }

}

// # 使用类型别名创建同义类型
type Kilometers = i32;
// 针对一个长类型进行使用别名，让代码不至于那么混乱
type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f:Thunk){}
fn returns_long_type()-> Thunk{
    Box::new(||())
}
// 使用已定义的 type Result<T> = Result<T,std::io::Error>;
pub trait Write{
    fn write(&mut self,buf:&[u8])->Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self,buf:&[u8])-> Result<()>;
    fn write_fmt(&mut self,fmt:Arguments)-> Result<()>;
}

// # 用不返回的Never类型
// 空类型 ! 更倾向叫 never类型：因为它在从不返回的函数中充当返回值的类型
fn panic() -> ! { // 发散函数
    panic!("panic!!");
}
// pannic！宏实现了never类型。参考Option的定义
// impl<T> Option<T> {
//     pub fn unwrap(self) -> T{
//         match self {
//             Some(val) => val,
//             None => pannic!("called `Option::unwrap()` on a `None` value")
//         }
//     }
// }