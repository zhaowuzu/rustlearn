use std::ops::Add;

pub fn demo(){
    // # 运算符重载
    assert_eq!(Point{x:1,y:0} + Point{x:2,y:3},Point{x:3,y:3})
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