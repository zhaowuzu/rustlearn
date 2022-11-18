mod generics_test;
mod trait_test;
use trait_test::Summary; // 这个地方要把接口给带过来，否则确实找不到对应的方法
use std::fmt::Display;
use std::fmt::Debug;

fn main(){
    // // 泛型demo
    //  generics_test::demo();
    //
    // // trait 的 demo
    // trait_demo

    // 生命周期的展示demo
    lifttime_demo()
}

/******************************生命周期的展示demo************************************/
fn lifttime_demo(){
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
}
// longest 返回最大的，无法判断最后返回的时那个参数，所以就都加上了
fn longest<'a>(x:&'a str,y:&'a str)-> &'a str { //未加'a 的报错：expected named lifetime parameter
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
// 只返回第一个，所以让参数1和返回结果生命周期是一样的就好了
fn first<'a>(x:&'a str,y:& str)-> &'a str {
        x
}
// 来看个有错误的
// fn no<'a>(x:&str,y:&str)-> &'a str {
//     let result = String::from("really long string");
//     result.as_str() // returns a reference to data owned by the current function 返回对当前函数拥有的数据的引用
// }
// 这个就没问题了，这个不是引用，而是一个数据，会把所有权转移给调用者
fn no1(x:&str,y:&str)-> String {
    let result = String::from("really long string");
    result
}
/******************************trait的demo和相关样例展示************************************/
fn trait_demo(){
    // trait的demo
    let tweet = trait_test::Tweet{
        username:String::from("horse_ebooks"),
        content:String::from("of course,as you probabley already know,people"),
        reply:false,
        retweet:false,
    };
    println!("1 new tweet:{}",tweet.summarize_author());
    println!("{}",tweet.summarize());
    let article =  trait_test::NewsArticle{
        headline:String::from("Penguins win the Stanley Cup Championship"),
        location:String::from("Pittsbirgh,PA,USA"),
        content:String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
        author:String::from("Iceburgh"),
    };
    println!("New article available!{}",article.summarize_author());
    println!("{}",article.summarize());

    notify(article);


    //let p1 = trait_test::Pair { x:6.0, y:4.0}; 不能通过这样直接创建
    // let p2 = trait_test::Pair::new(tweet,tweet); //`Tweet` doesn't implement `Debug`  不能通过编译，相当于用trait限定了对应结构。
    let p2 = trait_test::Pair::new(6,4);
    println!("{:?}",p2);

}

pub fn notify(item :impl Summary){ // impl Summary 是语法糖，真正的是：notify<T:Summary>(item :T)
    println!("Breaking news!{}",item.summarize_author())
}

// 试着比较一下这2个的不同
pub fn motify1(item1 :impl Summary,item2: impl Summary){} //只要是实现了Summary就可以
pub fn motify2<T:Summary>(item1:T,item2:T){}// 必须是实现了Summary的同一种类型

// 使用多个trait
pub fn notify3(item1:impl Summary+Display){} // 等价 notify3<T:Summary+Display>(item:T)

// 用Where整理多trait
pub fn some_function<T:Display+Clone,U:Clone+Debug>(t:T,u:U){} // 原来的
pub fn some_function_where<T,U>(t:T,u:U) // 整理后的
    where T:Display+Clone,
    U:Clone+Debug
{}

// 返回值使用trait
pub fn returns_summarizable(switch :bool) ->impl Summary {  // 只能返回一个实现了Summary的类型，不能存在分支可以返回不同类型的情况。不然无法通过编译器 `if` and `else` have incompatible types
    // if switch {
    //     trait_test::Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("of course,as you probabley already know,people"),
    //         reply: false,
    //         retweet: false,
    //     }
    // }else{
    //     trait_test::NewsArticle{
    //         headline:String::from("Penguins win the Stanley Cup Championship"),
    //         location:String::from("Pittsbirgh,PA,USA"),
    //         content:String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
    //         author:String::from("Iceburgh"),
    //     }
    // }

    // 以上的通不过编译器

    trait_test::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course,as you probabley already know,people"),
        reply: false,
        retweet: false,
    }
}