/// 餐馆前厅
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){
            super::super::eat_at_restaurant();
        }

        fn seat_at_table(){}
    }

    pub mod serving{
        fn take_order(){
            super::hosting::add_to_waitlist();
        }

        pub fn server_order(){}

        fn take_payment(){}


    }
}
/// 参观后厅
mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::front_of_house::serving::server_order(); // super 其实就是mod模块的上级模块，等价与 ../  这种操作方法
    }

    fn cook_order(){}

    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String,
    }

    impl Breakfast{
        pub fn summer(toast:&str)-> Breakfast {
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer{
        Soup,
        Salad,
    }
}

pub use crate::front_of_house::hosting; // 引用的时一个绝对路径，关键字 crate
//use self::front_of_house::hosting; // 引用的是一个相对路径,关键字 self

/// 客户在餐厅吃饭
pub fn eat_at_restaurant(){
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // use的使用
    hosting::add_to_waitlist();

    // 开始吃夏天早餐了
    // 选择黑麦面包作为夏季早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 修改我们想要的面包类型
    meal.toast = String::from("Wheat");
    println!("I`d like {} toast plase",meal.toast);

    // 接下来的这一行时无法通过编译，我们不能看到或更换随着食物附带的季节性水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("order1 {:?},order2 {:?}",order1,order2);

}

