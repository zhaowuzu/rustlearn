use std::cmp::PartialOrd;

// 注意：过多的泛型会使代码难以阅读，通常来讲，当你需要在代码中使用泛型时，可能意味着你的代码需要重构为更小的片段.
// RUST 泛型在性能上和使用具体类型的代码进行对比不会有任何速度上的差异。 会在编译的时候进行单态化，运行及其高效。

// 泛型结构体
// // 所以类型一样的
// struct Point<T> {
//     x:T,
//     y:T,
// }
// 类型可以不一样的
struct Point<T,U> {
    x:T,
    y:U,
}
// 泛型方法
impl<T,U> Point<T,U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
    // 只是简单的取值，不是做引用啥的
    fn val(self) -> (T,U) {
        (self.x,self.y)
    }
    fn mixup<V,W>(self,other:Point<V,W>)-> Point<T,W> {
        Point{
            x: self.x,
            y:other.y,
        }
    }
}

// 泛型枚举
// 参考一：标准库 Option
// enum Option<T> {
//     Some(T),
//     None,
// }
// 参考二：标准库 Result
// enum Result<T,E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    // 原始版：在数字列表中找最大值
    let mnumber_list = vec![34, 50, 25, 100, 65];

    let mut largest1 = mnumber_list[0];

    for &number in mnumber_list.iter() {
        if number > largest1 {
            largest1 = number;
        }
    }

    println!("The largest number is {}", largest1);

    // 函数抽象版
    let largest2 = largest(&mnumber_list);
    println!("The largest number is {}", largest2);

    // 函数抽象泛型版 不可用的反面教材
    // let largest3 = largest_generics(&mnumber_list);
    // println!("The largest number is {}",largest3);
    // let largest4 = largest_generics(&vec![2.1,3.5,9.9,4.6]);
    // println!("The largest number is {}",largest4);

    // 结构体泛型，看着还不错
    let interger = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let wnot_work = Point { x: "Hello", y: 4.0 }; // 第一种通不过：不同通过编译的 expected integer, found floating-point number  第二种通过
    println!("p.x = {}",wnot_work.x());  // 调用一个泛型的方法
    println!("mixup= {:?}",wnot_work.mixup(interger).val())  // 调用一个泛型的方法
}

fn largest(list:&[i32]) -> i32 {
    let mut largest = list[0];
    for &mumber in list.iter() {
        if mumber > largest {
            largest = mumber
        }
    }
    largest
}

// 泛型版  一个反面教材 无法通过编译的
// fn largest_generics<T>(list:&[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest { // 不是所以的已知类型都有比较方法，所以这里会报错 binary operation `>` cannot be applied to type `T`
//             largest = item
//         }
//     }
//     largest
// }

