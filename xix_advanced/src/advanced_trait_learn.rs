use std::ops::Add;
use std::fmt;
use std::fmt::{Display, Formatter, write};

pub fn demo(){
    // # 运算符重载
    assert_eq!(Point{x:1,y:0} + Point{x:2,y:3},Point{x:3,y:3});
    // # 相同的trait名称的方法：消除歧义的完全限定语法
    let person = Human;
    person.fly(); // 默认执行的是自己定义的fly方法
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);//与person.fly();等价
    <Human as Pilot>::fly(&person); // 简写：Pilot::fly(&person);

    println!("A baby dog is called a {}",Dog::baby_name());
    println!("A baby dog is called a {}",<Dog as Animal>::baby_name());// 完全限定

    //# 超trait：trait依赖其它的trait的方法
    let pp = Point1{x:20,y:54};
    pp.outline_print(); // 会打印被包裹起来的的样子

    //# 使用newtype模式在外部类型上实现外部trait
    let w = Wrapper(vec![String::from("hello"),String::from("world")]);
    println!("w = {}",w)
}
// 找一个标准库中的样例
pub trait Iterator{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter{}
// 关联类型
impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
// 泛型
pub trait Iterator_generic<T> {
    fn next(&mut self) -> Option<T>;
}

// # 运算符重载
#[derive(Debug,PartialEq)]
struct Point{
    x:i32,
    y:i32,
}
impl Add for Point{
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x:self.x + rhs.x,
            y:self.y + rhs.y,
        }
    }
}
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters{
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0*1000))
    }
}

// # 相同的trait名称的方法：消除歧义的完全限定语法
trait Pilot{
    fn fly(&self);
}
trait Wizard{
    fn fly(&self);
}

struct Human;
impl Pilot for Human {
    fn fly(&self) {
       println!("This is your captain speaking.")
    }
}
impl Wizard for Human {
    fn fly(&self) {
       println!("Up");
    }
}
impl Human{
    fn fly(&self){
        println!("*waving arms furiously");
    }
}

// 缺少&self的方法
trait Animal{
    fn baby_name() -> String;
}
struct Dog;

impl Dog {
    fn baby_name() -> String{
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
// 重点：完全限定语法为：
// <Type as Trait>::function(receiver_if_method,next_arg,...);

//# 超trait：trait依赖其它的trait的方法
trait OutlinePrint:fmt::Display{
    fn outline_print(&self){
        let output = self.to_string();
        let len = output.len();
        println!("{}","*".repeat(len + 4));
        println!("*{}*"," ".repeat(len + 2));
        println!("* {} *",output);
        println!("*{}*"," ".repeat(len + 2));
        println!("{}","*".repeat(len + 4));
    }
}
struct Point1{
    x:i32,
    y:i32,
}

impl Display for Point1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"({},{})",self.x,self.y)
    }
}

impl OutlinePrint for Point1 {}

//# 使用newtype模式在外部类型上实现外部trait
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"[{}]",self.0.join(", "))
    }
}