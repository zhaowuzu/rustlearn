use std::collections::HashMap; // 引用了这个结构体
use std::collections::hash_map;

fn main() {
    println!("Hello, world111111111!");
    let mut map = HashMap::new();
    map.insert(1,2);
    println!("{}",map[&1]);// 这个key的表示是有一定的要求的。

    let mut map1 = hash_map::HashMap::new();
    map1.insert("a","anb");
    println!("{}",map1["a"]);// 这个key的表示是有一定的要求的。
}
