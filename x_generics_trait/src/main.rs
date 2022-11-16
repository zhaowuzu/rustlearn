use std::cmp::PartialOrd;

fn main(){
    // 原始版：在数字列表中找最大值
    let mnumber_list = vec![34,50,25,100,65];

    let mut largest1 = mnumber_list[0];

    for &number in mnumber_list.iter() {
        if number > largest1 {
            largest1 = number;
        }
    }

    println!("The largest number is {}",largest1);

    // 函数抽象版
    let largest2 = largest(&mnumber_list);
    println!("The largest number is {}",largest2);

    // 函数抽象泛型版
    let largest3 = largest_generics(&mnumber_list);
    println!("The largest number is {}",largest3);
    let largest4 = largest_generics(&vec![2.1,3.5,9.9,4.6]);
    println!("The largest number is {}",largest4);

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

// 泛型版
fn largest_generics<T>(list:&[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}