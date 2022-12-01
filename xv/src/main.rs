use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::List::{Cons, Nil};
use std::ops::Deref;
use crate::ListRC::{ConsRC,NilRC};
use crate::List_RefCell_Rc::{Cons_RefCell_Rc,Nil_List_RefCell_Rc};
use crate::List_Cycle::{Cons_Cycle,Nil_Cycle};

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
    println!("CustomSmartPointer created.");

    // Rc的使用
    let a = Rc::new(ConsRC(5,
                Rc::new(ConsRC(10,
                   Rc::new(NilRC)))));
    println!("count after creating a= {}",Rc::strong_count(&a)); // strong_count强引用计数，weak_count弱引用计数
    let b = ConsRC(3,Rc::clone(&a)); // 不会深拷贝，只会增加计数
    //let c = ConsRC(4,a.clone()); // Rc::clone 的另一种写法，但是不是惯用写法 a.clone可能会走深拷贝？？   如果你只是要增加引用计数，那就用Rc::clone
    println!("count after creating b= {}",Rc::strong_count(&a));
    {
        let c = ConsRC(4,Rc::clone(&a));
        println!("count after creating c= {}",Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}",Rc::strong_count(&a));

    // RefCell<T>
   // let x = 5;
    //let y = &mut x; // cannot borrow `x` as mutable, as it is not declared as mutable.
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons_RefCell_Rc(Rc::clone(&value),Rc::new(Nil_List_RefCell_Rc)));// clone让a和value同时拥有了5的所有权
    // 把a接在b和c后面
    let b = Cons_RefCell_Rc(Rc::new(RefCell::new(6)),Rc::clone(&a));
    let c = Cons_RefCell_Rc(Rc::new(RefCell::new(10)),Rc::clone(&a));
    *value.borrow_mut() += 10;
    //value.borrow_mut() += 10; // binary assignment operation `+=` cannot be applied to type `RefMut<'_, i32>`
    println!("a after = {:?}",a);
    println!("b after = {:?}",b);
    println!("c after = {:?}",c);

    // 循环调用，溢出样例
    let a = Rc::new(Cons_Cycle(5,RefCell::new(Rc::new(Nil_Cycle))));
    println!("a initial rc count = {}",Rc::strong_count(&a));
    println!("a next item = {:?}",a.tail());
    let b = Rc::new(Cons_Cycle(10,RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation= {}",Rc::strong_count(&a));
    println!("b initail rc count = {}",Rc::strong_count(&b));
    println!("b next item = {:?}",b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // 把a的第二项从原来的Nil_Cycle变成b，就构成了个环
    }
    println!("b rc count after changing a = {}",Rc::strong_count(&b));
    println!("a rc count after changing a = {}",Rc::strong_count(&a));
    // 取消下面的注释便可以观察到循环引用，它会造成栈的溢出
    //println!("a next item = {:?}",a.tail())

    // 弱引用
    let leaf = Rc::new(Node{
       value:3,
        parent:RefCell::new(Weak::new()),// 只能先来一个空的了
        children:RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}",leaf.parent.borrow().upgrade());
    println!("leaf strong = {},weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
    let branch = Rc::new(Node{
       value:5,
        parent:RefCell::new(Weak::new()),
        children:RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);// 给leaf挂上父节点
    println!("branch strong = {},weak = {}",Rc::strong_count(&branch),Rc::weak_count(&branch));
    println!("leaf strong = {},weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
    println!("leaf parent = {:?}",leaf.parent.borrow().upgrade());
    println!("leaf strong = {},weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
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

// # 引用计数的智能指针Rc<T> :Reference counting(引用计数)
// 使用场景一：当你希望将堆上的一些数据分享给程序多个部分同时使用，而又无法在编译期确定哪个部分会最后释放。
// 注意：Rc<T>只能被单线程场景
enum ListRC {
    ConsRC(i32,Rc<ListRC>),
    NilRC,
}

// # RefCell<T>和内部可变性模式
// 注意：RefCell<T>只能单线程
// 关键对比：Box<T>,Rc<T>还是RefCell<T>
// 1： Rc<T>允许一份数据有多个持有者，而Box<T>,RefCell<T>都只有一个所有者。
// 2： Box<T>允许在编译时检查的可变或者不可变借用，Rc<T>仅允许编译时检查的不可变借用，RefCell<T>允许运行时检查的可变或不可变借用。
// 3： 由于RefCell<T>允许我们在运行时检查可变借用，所以即便RefCell<T>本身时不可变的，我们仍然能够更改其中存储的值。
// 还要其它：
// Cell<T>:相比与RefCell<T>;RefCell<T>是通过借用来实现内部数据的读写，Cell<T>选择通过复制来访问数据。
// Mutex<T>:它被用于实现跨线程情形下的内部可变性模式。
#[derive(Debug)]
enum List_RefCell_Rc{
    Cons_RefCell_Rc(Rc<RefCell<i32>>,Rc<List_RefCell_Rc>),
    Nil_List_RefCell_Rc,
}



// # 循环引用样例，会导致内存不能够释放
#[derive(Debug)]
enum List_Cycle{
    Cons_Cycle(i32,RefCell<Rc<List_Cycle>>),
    Nil_Cycle,
}

impl List_Cycle {
    fn tail(&self) -> Option<&RefCell<Rc<List_Cycle>>> {
        match self {
            Cons_Cycle(_,item) => Some(item),
            Nil_Cycle=>None,
        }
    }
}


// # Weak<T>智能指针的使用
// 相较于Rc::clone的强引用而言，Rc::downgrade得到的是一个Weak的弱引用,
// 弱引用的计数不需要为0，只有强引用的计数为0了，生命周期就结束了。
#[derive(Debug)]
struct Node{
    value:i32,
    parent:RefCell<Weak<Node>>, // 父节点事弱引用，子节点不应该拥有父节点。
    children:RefCell<Vec<Rc<Node>>>,// 共享所以子节点，所以用Rc包了一下。希望灵活修改节点的父子关心，所以用RefCell包了一下。
}
