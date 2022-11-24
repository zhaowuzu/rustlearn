use std::thread;
use std::time::Duration;
use std::hash::Hash;
use std::collections::HashMap;

// 存储一个闭包和可选结构值的结构体
struct Cacher<T,U>
where T:Fn(U) ->U,
      U:Eq+Hash
{
    calculation:T,
    //value:Option<u32>,
    map:HashMap<U,U>,
}

// 未执行的时候执行，执行后直接取值.   这样在某个需要耗时的地方比如初始化操作只会别调用一次了。
// 记忆化，惰性求值。只有在被第一次使用的时候才会去求值，而不是预先定义好，放在那里，浪费内存。
impl<T,U> Cacher<T,U>
where T:Fn(U)->U,  // 闭包有Fn,FnMut,FnOnce
U:Eq+Hash+Copy
{
    fn new(calculation:T)->Cacher<T,U>{
        Cacher{
            calculation,
            //value:None,
            map:HashMap::new(),
        }
    }
    fn value(&mut self,arg:U)-> U {

        // match self.value {
        //     Some(v) =>v ,
        //     None => {
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         v
        //     }
        // }
        match self.map.get(&arg) {
            Some(v) => *v ,
            None => {
                let v = (self.calculation)(arg);
                self.map.insert(arg,v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // 闭包的错误用法
    // let example_closure = |x| x ;
    // let s = example_closure(String::from("hello"));
    // let s = example_closure(5); // arguments to this function are incorrect

    // 闭包捕获上下文环境
    // 正向：
    let x = 4;
    let equal_to_x = |z| z==x;
    let y = 4;
    assert!(equal_to_x(y));
    // 反向案例，函数
    //fn equal_to_x(z:i32)-> bool{z==x};

    // let y = 6;
    // assert!(equal_to_x(y)); // 会报pannic

    // move 闭包，强制获取所有权
    let x = vec![1,2,3];
    let equal_to_x = move |z| z==x;
    // println!("can't use x here:{:?}",x); // value borrowed here after move 编译不会通过的
    let y = vec![1,2,3];
    assert!(equal_to_x(y));
}

// fn simulated_expensive_calculation(intensity:u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity:u32,random_number:u32){
    //let expensive_result= simulated_expensive_calculation(intensity);
    // 闭包来了
    // let expensive_result = |num|{ // 正常闭包是不需要标注类型的。不过不是不可以
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // let expensive_result = |num:u32|->u32{ // 不是很建议
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // 结构体升级
    let mut expensive_result = Cacher::new(|num|{
        println!("calculating slowly...");
             thread::sleep(Duration::from_secs(2));
             num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!", // 今天做{}个俯卧撑
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!", // 接下来做多少个{}仰卧起坐
            expensive_result.value(intensity+3)
        );
        println!(
            "Next, do {} other!", // 接下来做多少个{}仰卧起坐
            expensive_result.value(intensity)
        );
    }else{
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!"); // 今天休息一下吧！别忘了补充水分
        }else{
            println!(
                "Today, run for {} minutes!", // 今天跑步{}分钟
               // expensive_result.value(String::from("至少5")) // mismatched types
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values(){
    let mut c = Cacher::new(|a|a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2,2);
}