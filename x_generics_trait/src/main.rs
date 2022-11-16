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