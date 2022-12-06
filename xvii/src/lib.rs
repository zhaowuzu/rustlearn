pub struct AveragedCollection {
    list :Vec<i32>,
    average : f64,
}

impl AveragedCollection{
    pub fn add(&mut self,value:i32){
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average

    }

    fn update_average(&mut self){
        let total:i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

/************************************使用trait定义公共方法来模拟继承*****************************************/

/// 定义一个GUI的公共Draw tarit，主要表示写
/// Draw对象被传入的时候要保证对象安全，满足以下两条规则就可以保证tarit是对象安全的：
//  1. 方法的返回类型不是self.
//  2. 方法中不包含任何泛型参数
pub trait Draw {
    fn draw(&self);
}

/// 定义了一个动态数组，内容实现了draw trait.而且使用了Box指针主动放进堆上
/// 第一种：非同质集合，意味集合中可以是Button，也可以是Text
/// 意思有 鸭子类型
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
    //pub components: Vec<Box<dyn Clone>> // 编译器会报错，因为Clone trait不是线程安全的。 fn clone(&self) -> Self;
}
impl Screen {
    /// 逐一调用components中每个元素draw方法
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

/// 第二种：同质集合，意味集合中只能是Button，或Text
pub struct ScreenD<T:Draw> {
    pub components: Vec<T>
}
impl<T> ScreenD<T>
where T:Draw{
    /// 逐一调用components中每个元素draw方法
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

// 一些tarit的实现
/// 一个按钮
#[derive(Debug)]
pub struct  Button {
    pub width:u32,
    pub height:u32,
    pub label:String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("我是一个按钮：{:?}",self)
    }
}

/// 一个选额框
#[derive(Debug)]
pub struct  SelectBox {
    pub width:u32,
    pub height:u32,
    pub options:Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("我是一个选额框：{:?}",self)
    }
}
