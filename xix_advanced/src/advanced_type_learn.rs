pub fn demo(){
    // # 使用类型别名创建同义类型
    let x:i32 = 5;
    let y:Kilometers = 5;
    println!("x + y = {}",x+y); // 不同类型可以加，其实最内部是一样的类型
}

// # 使用类型别名创建同义类型
type Kilometers = i32;