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
    // print!("forever");
    // loop {
    //     print!("and ever");
    // }

    // # 动态大小类型和sized trait
    //let s1:str = "Hello there"; // 是不能直接定义str的
    //let s1:str = "How's it going?"; // 是不能直接定义str的

    // # 高级函数与闭包
    // 函数指针
    let answer = do_twice(add_one,5);
    println!("The answer is :{}",answer);
    // 同事支持闭包和函数指针
    let list_of_numbers = vec![1,2,3];
    let list_of_strings:Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string()) // 闭包
        .collect();
    let list_of_strings:Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string) // 函数
        .collect();
    // 构造器的方法
    enum Status{
        Value(u32),
        Stop,
    }
    let list_of_statuses:Vec<Status> =
        (0u32..20)
        .map(Status::Value)
        .collect();
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

// # 动态大小类型和sized trait
fn generic<T>(t:T){} // 这种定义限制了编译的时候的T是已知的大小
// 上面定义会被隐式转换为下面的定义
//fn generic<T:sized>(t:T){}
fn generic1<T:?Sized>(t:&T){} // 通过？sized 解除对已知大小的限制，因为参数T可能不是Sized，所以要使用&T做指针引用，来高度编译器的大小（其实转换成了指针的大小）

// # 高级函数与闭包
// 函数指针
// fn类型：函数指针:实现了Fn,FnMut,FnOnce
// Fn:Fn闭包trait
fn add_one(x:i32) -> i32 {
    x+ 1
}
// 传入的是一个函数指针
fn do_twice(f:fn(i32)->i32,arg:i32)-> i32 {
    f(arg) + f(arg)
}
// 返回闭包？？
// fn returns_closure() -> Fn(i32)-> i32 { // trait objects must include the `dyn` keyword: Rust无法推断出自己需要多大空间来存储此处返回的闭包
//     |x| x+1
// }
// 优化
fn returns_closure() -> Box<dyn Fn(i32)-> i32> {
     Box::new(|x| x+1)
}

// # 宏：元编程范式
/*
1. macro_rules! 构造的声明宏（declarative macro），也叫“宏模板”
2. 过程宏（procedural macro）
    1. 用于结构体或枚举的自定义#[derive]宏，它可以指定随derive属性自动添加的代码。
    2. 用于任意条目添加自定义属性的属性宏。
    3. 看起来类似于函数的函数宏，它可以接收并处理一段标记（token）序列。
*/
// 仿照vec!做的简化例子
#[macro_export]   // 这个宏会在它所处的包被引入作用域后可用，缺少这个标注的宏则不能被引入作用域
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
// // // 过程宏
// // #[some_attribute]
// // pub fn some_name(input:TokenStream) -> TokenStream {
//
// }