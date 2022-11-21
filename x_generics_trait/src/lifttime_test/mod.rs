
/******************************生命周期的展示demo************************************/
pub fn lifttime_demo(){
    // 尝试在值离开作用域时使用指向它的引用
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x; // borrowed value does not live long enough
    //     }
    //     println!("r:{}",r);
    // }
    // 第一种变体 都行
    {
        let r;
        let x = 5;
        r = &x;
        println!("r:{}",r);
    }
    // 第二种变体
    {
        let x = 5;
        let r;
        r = &x;
        println!("r:{}",r);
    }

    //函数中的泛型生命周期
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(),string2);
    println!("The longest string is {}",result);
    // 正常被允许
    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result1 = longest(string3.as_str(),string4.as_str());
        println!("The longest string is {}",result1);
    }
    // 会出错
    // let string5 = String::from("long string is long");
    // let result2;
    // {
    //     let string6 = String::from("xyz");
    //     result2 = longest(string5.as_str(),string6.as_str());// borrowed value does not live long enough
    // }
    // println!("The longest string is {}",result2);

    // result3 获取了对应的所有权
    let result3 = no1(string1.as_str(),string2);
    println!("The longest string is {}",result3);

    // 测试结构体生命周期的标注
    let  novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("ImportantExcerpt is {:?}",i);
    // 错误的范例
    let mut y = ImportantExcerpt{part:"哦"};
    {
        let  novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
        y = ImportantExcerpt { part: first_sentence };
        //println!("ImportantExcerpt is {:?}",y);  // 用在这里时正确的。
    }
   //println!("ImportantExcerpt is {:?}",y); // 用在这里就错了，超过生命周期了

    // 显示做标注：镜头生命周期
    let s:&'static str = "I have a static lifttime";
}

// longest 返回最大的，无法判断最后返回的时那个参数，所以就都加上了
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { //未加'a 的报错：expected named lifetime parameter
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 只返回第一个，所以让参数1和返回结果生命周期是一样的就好了
fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// 来看个有错误的
// fn no<'a>(x:&str,y:&str)-> &'a str {
//     let result = String::from("really long string");
//     result.as_str() // returns a reference to data owned by the current function 返回对当前函数拥有的数据的引用
// }
// 这个就没问题了，这个不是引用，而是一个数据，会把所有权转移给调用者
fn no1(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result
}


/* 省略生命周期的三条规则
规则一：每一个引用都会拥有自己的生命周期参数。换句话说，单参数拥有一个生命周期参数：fn foo<'a>(x:&'a i32);双参数函数拥有两个不同的生命周期参数：fn foo<'a,'b>(x:&'a i32,y:&'b i32);以此类推。
规则二：当只存在一个输入生命周期参数时，这个生命周期会被赋予所有输出生命周期参数，例如fn foo<'a>(x:&'a i32) -> &'a i32。
规则三：当拥有多个输入生命周期参数，而其中一个是&self或&mut self时，self的生命周期会被赋予给所有的输出生命周期参数。这条规则使得方法更易阅读和编写，因为它省略了一些不必要的符合。
方法和函数要做区分哈，方法主要是结构体的impl附属方法，函数就会全局定义的一些函数。
*/
// 结构体生命周期标注
#[derive(Debug)]
struct ImportantExcerpt<'a> { // 生命周期的标注
    part :&'a str, // 结构体存储一个引用（其实就是go中的指针）；需要为每一个引用添加生命周期的标注
}
impl<'a> ImportantExcerpt<'a>{
    fn level(&self) -> i32 { // 根据规则1 参数的生命周期标注会被忽略
        3
    }
}
impl<'a> ImportantExcerpt<'a>{
    fn announce_and_return_part(&self,announcement:&str) -> &str { // 根据规则1，3 参数的生命周期标注会被忽略
        println!("Attenion please:{}",announcement);
        self.part
    }
}
// impl<'a> ImportantExcerpt<'a>{
//     fn announce_and_return_part1(&self,announcement:&str) -> &str { // 这个就是个无法通过编译的。因为根据1和3之后发现不符合，因为返回的不是&self的东西，而是第二个参数。就需要做显示标注了
//         println!("Attenion please:{}",announcement);
//         announcement
//     }
// }


// 静态生命周期 'static
// 所有的字符串字面量都拥有'static生命周期。
// 显示做标注：
//let s:&'static str = "I have a static lifttime";



