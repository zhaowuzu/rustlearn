pub fn demo(){
    // 字符串是基于UTF-8编码的
    println!("---------------------------String------------------------------");
    // 使用new创建空字符串
    let mut s = String::new();
    // 使用Display trait的类型调用to_string方法创建
    let data = "initial contents 1";
    let s1 = data.to_string();
    let s2 = "initial contents 2".to_string();
    // 使用字面量创建
    let s3 = String::from("initial contents 3");
    println!("{},{},{},{}",s,s1,s2,s3);
    s = "initial contents 4".to_string();
    println!("{},{},{},{}",s,s1,s2,s3);

    // 使用push_str或push 添加
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}",s);
    let s2 = "bar";
    s.push_str(s2); // 这个push_str不会获取s2的所有权。
    println!("{},{}",s,s2); // s2被正常打印了
    s.push('l'); // 增加单个字符
    println!("{}",s);
    // + 拼接
    let sf = String::from("Hello, ");
    let sl = String::from("world!");
    let sr = sf + &sl; // 注意这里的sf已经被移动且再也不能被使用了
    //let sr = &sf + &sl; // 直接编译不会通过 add(self,s:&str) -> String
    //let sf = sf+ &sl; // 可以正常通过
    println!("?+{}={}",sl,sr);
    // 使用format!宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}",s1,s2,s3);// 不会剥夺s1,s2,s3的所有权
    println!("{}",s);

    // 索引
    // let h = s[0]; // 编译错误：`String` cannot be indexed by `{integer}`
    let len1 = String::from("Hola").len();// 4
    let len2 = String::from("ЗДpabc").len();// 8
    let hello = "ЗДpabc";
    // let h = &hello[0]; // 编译不过
    let s = &hello[0..2];// 3和1不合法会允许panic因为ЗД每一个占位2字节
    println!("{}，{},{}",len1,len2,s);
    // 遍历 使用chars来显示字符，使用bytes来显示字节
    for c in hello.chars() {
        println!("{}",c);
    }
    for c in hello.bytes() {
        println!("{}",c);
    }


}