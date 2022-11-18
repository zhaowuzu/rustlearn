mod generics_test;
mod trait_test;
use trait_test::Summary; // 这个地方要把接口给带过来，否则确实找不到对应的方法
use std::fmt::Display;
use std::fmt::Debug;

fn main(){
    // 泛型demo
     generics_test::demo();

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