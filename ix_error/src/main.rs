use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;
use std::fs;
use std::error::Error;

// 一种高级main,可以使用？
fn main() -> Result<(),Box<dyn Error>>{
    let f = File::open("hello.txt")?;
    Ok(())
}

// fn main() {
//     // 不可恢复的错误
//     // panic!("crash and burn")
//
//     // 日常工作中可能存在的一种panic
//     // let v = vec![1,2,3];
//     // v[99];
//
//     // 使用match来进行匹配对应的枚举值
//     // let f = File::open("hello.txt");
//     // let f = match f {
//     //     Ok(file) => file,
//     //     Err(error) => match error.kind() { // error = io::Error  可以通过kind方法获得ErrorKind的值
//     //         ErrorKind::NotFound => match File::create("hello.txt") { // NotFound标准困提供
//     //             Ok(fc) => fc,
//     //             Err(e) => panic!("There to creat file but there was a problem:{:?}",e),
//     //         },
//     //         other_error => panic!("There was a problem opening the file:{:?}",other_error),
//     //     },
//     // };
//
//     // 不使用match来匹配错误，进阶版,有经验的程序员会这样写，减少match的数量
//     // let f = File::open("hello.txt").map_err(|error| {
//     //     if error.kind() == ErrorKind::NotFound {
//     //         File::create("hello.txt").unwrap_or_else(|error|{ // 13章才会讲到unwrap_or_else
//     //          panic!("Tried to crate file but there was a problem:{:?}",error)
//     //         }
//     //         )
//     //     }else{
//     //         panic!("There was a problem opening the file:{:?}",error)
//     //     }
//     // }
//     // );
//
//     //let f = File::open("hello.txt").unwrap(); // OK时返回ok内部的值。如果错误就直接调用panic
//     //let f = File::open("hello.txt").expect("Failed to open hello.txt"); //与unwrap差别在可以传入错误提示信息
//
//    // let name = read_username_from_file();
//     //let name = read_username_from_file_v2();
//    // let name = read_username_from_file_v3();
//     let name = read_username_from_file_v4();
//     match name {
//         Ok(s) =>  println!("{},",s),
//         Err(e) => panic!("read_username_from_file fail:{:?}",e)
//     }
// }

// 未使用错误传播的快捷方式 ？
fn read_username_from_file() -> Result<String,io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(fc) => fc,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    // 由于是最后一个表达式，就不需要进行显示return
    match f.read_to_string(&mut s) {// 可变的引用，就赋值给外部的是了
        Ok(_) => Ok(s),
        Err(e) => Err(e),// error 类型正好是 io::Error
    }
}

// ? 只能配合Result的返回类型中，其它不适合使用
fn read_username_from_file_v2()-> Result<String,io::Error> {
    let mut f = File::open("hello.txt")?;// ?错误直接return ,正确继续往下执行

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_v3()-> Result<String,io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;  // 直接使用？做链式传播
    Ok(s)
}

// 这个方法说实话确实会短，但是也只是调用一个库而已。跟本章学习关系不大
fn read_username_from_file_v4()-> Result<String,io::Error> {
    fs::read_to_string("hello.txt")
}