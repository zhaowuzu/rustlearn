use std::collections::HashMap;

pub fn demo(){
    println!("---------------------------hash------------------------------");
    // 创建
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    println!("{:?}",scores);
    // 另一个比较有意思的东东
    let teams = vec![String::from("Blue"),String::from("Yellow")];
    let initial_scores = vec![10,50];
    let scores_pro: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}",scores_pro);
    // 再来一种
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    // field_name和field_value从这一刻开始失效，若尝试使用它们则会导致编译错误！
    println!("{:?}",map);

    // 访问值
    let score = map.get(&String::from("Favorite color"));
    match score {
        Some(val) => println!("{}",val),
        None => println!("呀，值没有哎"),
    }
    // 遍历
    for (key,value) in scores_pro{
        println!("{},{}",key,value);
    }

    // 更新键值
    // 相同键，直接复制
    let mut scores= HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Blue"),25);
    println!("{:?}",scores);
    // 先判空再插入
    scores.entry(String::from("Yellow")).or_insert(50);// 插入
    scores.entry(String::from("Blue")).or_insert(50);// 不插入
    println!("{:?}",scores);
    // 基于旧值改变新值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}",map);
}