// # 静态变量
// 访问一个不可变静态变量时安全的
static HELLO_WORLD:&str = "Hello,world";

/*
常量和不可变静态变量区别：
+ 静态变量的值在内存中拥有固定的地址，使用它的值总是会访问到同样的数据。
+ 常量则允许在任何被使用到的时候复制其数据。
另一个和静态变量的区别：
+ 静态变量是可变的。访问和修改静态变量是不安全的。
*/

// 从一个可变静态变量中读或写都是不安全的
static mut COUNTER:u32 = 0;
fn add_to_count(inc:u32){
    unsafe {
        COUNTER += inc;
    }
}

pub fn demo(){
    println!("name is:{}",HELLO_WORLD);

    // 从一个可变静态变量中读或写都是不安全的
    // 多个线程同时访问COUNTER很难保证没有数据竞争。可以使用智能指针进行包装
    add_to_count(3);
    unsafe { // 去掉：use of mutable static is unsafe and requires unsafe function or block
        println!("COUNTER:{}",COUNTER);
    }
}