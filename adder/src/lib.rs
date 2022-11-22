// 存放逻辑的

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
//  fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[derive(Debug)]
// pub struct Rectangle{
//     length: u32,
//     width: u32,
// }
// impl Rectangle {
//     pub fn can_hold(&self,other:&Rectangle) -> bool {
//         self.length > other.length && self.width > other.width
//     }
// }
//
// pub struct Guess{
//     value: u32,
// }
//
// impl Guess{
//     pub fn new(value:u32)-> Guess{
//         // 不引发pannic
//         // if value <1 || value > 100 {
//         //     panic!("Guess value must e between 1 and 100,got {}. ",value);
//         // }
//
//         // // 引发一个pannic
//         // if value <1  {
//         //     panic!("Guess value must e between 1 and 100,got {}. ",value);
//         // }
//
//         // 引发一个具体pannic
//         if value <1  {
//             panic!("Guess value must be greater than or equal to 1,got {}. ",value);
//         }else if value > 100 {
//             panic!("Guess value must be less than or equal to 100,got {}. ",value);
//         }
//         Guess{
//             value
//         }
//     }
// }

// 直接执行cargo test
//
// #[cfg(test)] // 只在执行cargo test命令时编译器才会执行，build不会执行的，会剔除掉
// mod tests {
//
//     use super::*;// 通配符 * 让外层模块所定义的全部内容在testes模块中可以
//
//     // // 用Result<T,E>编写测试
//     // #[test]
//     // fn it_works()->Result<(),String>{
//     //     if 2+2 == 5{
//     //         Ok(())
//     //     }else{
//     //         Err(String::from("two plus two does not equal four"))
//     //     }
//     // }
//
//     #[test] // 标识为一个测试函数  测试会成功
//     fn exploration() {
//         let result = add(2, 2);
//        // assert_eq!(2+2, 4); // assert_eq 断言宏相等
//         assert_ne!(result,4);// assert_eq 断言宏不相等
//     }
//
//     // #[test]
//     // fn another(){
//     //     panic!("Make this test fail");// 测试会失败
//     // }
//     //
//     // #[test]
//     // fn largest_can_hold_smaller(){
//     //     let larger = Rectangle{length:8,width:7};
//     //     let smaller = Rectangle{length:5,width:1};
//     //
//     //     assert!(larger.can_hold(&smaller));// 判断传入参数是否为true
//     // }
//     // #[test]
//     // #[ignore] // 当指明需要忽略的关键字后，该测试会被忽略.  不过剋有单独使用 cargo test -- --ignored 来执行这个测试的执行
//     // fn smaller_can_hold_largest(){
//     //     let larger = Rectangle{length:8,width:7};
//     //     let smaller = Rectangle{length:5,width:1};
//     //     assert!(smaller.can_hold(&larger),"Greeting did not contain name,value was '{}'","不正确"); // 判断出问题的，打印错误日志
//     // }
//     //
//     // #[test]
//     // //#[should_panic] // 普通的不对pannic进行校验
//     // #[should_panic(expected = "Guess value must be less than or equal to 100")] // 可以对pannic进行校验 如果是匹配上是可以通过的,就说明确实是这个错误
//     // fn greater_than_100(){
//     //     Guess::new(200); // tests::greater_than_100 stdout --- note: test did not panic as expected
//     // }
// }
