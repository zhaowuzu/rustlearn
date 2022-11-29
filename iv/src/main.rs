use crate::List::{Cons,Nil};
use std::ops::Deref;

fn main() {
    // 再堆上存储一个i32
    let b = Box::new(5); // 栈上有个指针，堆上有真实的数据5
    println!("b= {}",b);
    println!("b= {}",*b); // 这2个都一样，可以用解的也可以不用解的

    // 递归中的使用
    let _list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));

    // 解引用跟踪
    let x = 5;
    //let y = &x;
    let y = Box::new(x); // 这个和上一行一样
    assert_eq!(5,x);
    assert_eq!(5,*y); // 解引用，不使用*的话会被错：can't compare `{integer}` with `&{integer}`
    // 自检的仿照的智能指针
    let z = 6;
    let zb =  MyBox::new(z);
    assert_eq!(6,z);
    assert_eq!(6,*zb);// 未实现Deref tarit之前会报错：type `MyBox<{integer}>` cannot be dereferenced  ；实现之后类似与*(y.deref)
}

// 自建的一个list
enum List {
    Cons(i32,Box<List>), // 使用Box固定一下递归的大小，使其可以通过编译。
    Nil,
}

// Deref trait  解引用运算符

struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x:T) ->MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T>{
    type Target = T; // 关联类型
    fn deref(&self) -> &T{
        &self.0 // 返回一个指向值的引用，所以运行调用者通过*运算符访问值
    }
}