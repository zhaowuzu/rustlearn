use std::slice;
fn main() {
    // unsafe 的不安全超能力
    // 1. 解引用裸指针
    // 2. 调用不安全的函数或方法
    // 3. 访问或修改可变的静态变量
    // 4. 实现不安全trait

    // # 解引用裸指针
    /*
    裸指针与引用，智能指针的区别在于：
    1.允许忽略借用规则，可以同时拥有指向同一内存地址的可变和不可变指针，或者拥有指向同一个地址的多个可变指针。
    2.不能保证自己总是指向有效的内存地址。
    3.允许为空。
    4.没有实现任何自动清理机制
    */
    // 通过引用创建裸指针
    let mut num = 5;
    let r1 = &num as *const i32; // 不可变裸指针
    let r2 = &mut num as *mut i32;// 可变裸指针
    // 创建一个指向任意内存地址的裸指针
    let address = 0x012345;
    let r = address as *const i32;
    // 解引用 裸指针
    unsafe {
    // 去掉unsafe的错误：dereference of raw pointer is unsafe and requires unsafe function or block
        println!("r1 is: {}",*r1);
        println!("r2 is: {}",*r2);
        //println!("r2 is: {}",*r); // 由于是随便写的，所以找不到内容也就很正常了process didn't exit successfully:‘’(exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
    }

    // # 调用不安全函数或方法
    //dangerous();// Call to unsafe function requires unsafe function or block
    unsafe {
        dangerous();
    }
    // 标准库中使用split_at_mut的方法样例：
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);
    assert_eq!(a,&mut[1,2,3]);
    assert_eq!(b,&mut[4,5,6]);
}

// # 调用不安全函数或方法
unsafe fn dangerous(){}
// 标准库中使用split_at_mut的方法样例：
/*
let mut v = vec![1,2,3,4,5,6];
let r = &mut v[..];
let (a,b) = r.split_at_mut(3);
assert_eq!(a,&mut[1,2,3]);
assert_eq!(b,&mut[4,5,6]);
*/
// // 错误版
// fn split_at_mut(slice:&mut[i32],mid:usize) -> (&mut[i32],&mut[i32]){
//     let len = slice.len();
//     assert!(mid <= len);
//     (&mut slice[..mid],&mut slice[mid..])
// }
// 正确的，不安全版
fn split_at_mut(slice:&mut[i32],mid:usize) -> (&mut[i32],&mut[i32]){
    // 切片由一个指向数据的指针与切片长度组成
    let len = slice.len();// len获取切片长度
    let ptr = slice.as_mut_ptr();// as_mut_ptr 访问切片包含的裸指针，由于本身slice是mut的所以最终返回了一个*mut i32可变裸指针
    assert!(mid<= len);
    unsafe {
        (slice::from_raw_parts_mut(ptr,mid),slice::from_raw_parts_mut(ptr.offset(mid as isize),len -mid))
    }
}