pub fn demo(){
    println!("---------------------------vec!------------------------------");
    // 创建一个给定类型的动态数组
    let _v:Vec<i32> = Vec::new();
    // 创建一个自己推导类型的数组，且使用了vec!宏
    let mut v = vec![1,2,3];
    // 添加
    v.push(5);
    println!("{:?}",v);
    // 取出
    let pv = v.pop();
    println!("{:?}",pv); // 这里得到的数值，要去match判断是否等于NONE  值为Some(5)

    // 使用索引得到数组某个下标的值,这样操作，一旦越界，会直接导致程序崩溃退出“thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 5”
    let third: i32 = v[2]; // 2 种方法都可以，但是这种值就不会变了
    let third1: &i32 = & v[2]; // 这里拿到这个引用，就会导致不会再有人能拿到这个引用，去做变更

    println!("The third element is {}，{}",third,third1);
    // 使用get获取对应的值，数组越界也会被捕获执行None
    match v.get_mut(1) {
        Some(first) =>{
            *first += 12;
            println!("The third element is {}",first);
        },
        None => println!("There is nothird element"),
    }
    // 更改某个内容的值的写法应该是这样了。被我给发现了
    match v.get(1) {
        Some(first) => println!("The third element is {}",first),
        None => println!("There is nothird element"),
    }

    // 不可执行的操作，同时存在19和20行会违反所有权和借用规则
    //v.push(6);
    //println!("The third element is {}",third); // 这里会把&v[2]借用的作用域延续到这个地方，就会导致push异常。注销掉这一句，push就可以正常使用了。

    // 遍历可变动态数据并进行相应计算
    for i in &mut v{
        *i += 50; // * 是找到引用所指向的地址的值取出来
    }
    // 动态数组的遍历
    for i in &v {
        println!("{}",i)
    }

    // 利用枚举存放不同数据类型-示例
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in row {
        match i {
            SpreadsheetCell::Int(ii)=>println!("整数{}",ii),
            SpreadsheetCell::Float(ii)=>println!("浮点数{}",ii),
            SpreadsheetCell::Text(ii)=>println!("字符串{}",ii),
        };
    }
}// 里面定义的数组离开自己的作用域并随之被销毁