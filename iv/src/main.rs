use crate::List::{Cons,Nil};
use std::ops::Deref;

fn main() {
    // 再堆上存储一个i32
    let b = Box::new(5); // 栈上有个指针，堆上有真实的数据5
    println!("b= {}",b); // 这个就是用到了解引用转换，就不需要显示的使用*了
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

    // 函数解引用转换，需要传入的参数实现了Deref
    hello("赵通");
    let m = MyBox::new(String::from("Rust"));
    hello(&m);// 等价与没有解引用的写法 hello(&(*m)[..])

    // Drop trait
    // 变量的丢弃顺序与创建顺序正好相反
    let c = CustomSmartPointer{data:String::from("My stuff")};
    //c.drop();// explicit use of destructor method
    drop(c);// 这个是被允许的 std::mem::drop 提前丢弃值
    let d = CustomSmartPointer{data:String::from("other stuff")};
    println!("CustomSmartPointer created.")
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

// ## 函数的解引用转换
fn hello(name :&str) {
    println!("Hello,{}",name); // 利用了解引用转换，因为str实现了Deref
}
/*
解引用转换触发条件：
1.当T:Deref<Target=U>时，允许&T转换为&U.如果类型T实现了类型U的Deref trait,那么&T就可以直接转换为&U。
2.当T:DerefMut<Taeget=U>时，允许&mut T转换为&mut U.
3.当T:Deref<Target=U>时，允许&mut T转换为&U.可变应用转换成了一个不可变应用，绝不会逆转。
*/


// #Drop trait
struct CustomSmartPointer{
    data:String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) { // 析构函数
        println!("Dropping CustomSmartPointer with data`{}`!",self.data);
    }
}