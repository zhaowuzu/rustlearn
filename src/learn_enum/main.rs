fn main(){
    let four = IpAddr::v4(127,0,0,1);
    let six = IpAddr::v6("::1".to_string());
    route(&four);
    println!("{:?}\n{:?}", four, six);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number :Option<i32> = None;
    let absent_number1 :Option<i32> = Some(6);
    println!("{:?},{:?},{:?},{:?}",some_number,some_string,absent_number,absent_number1);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?},{:?},{:?}",five,six,none);
     let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value{
        println!("挺好");
    }

    let coin = Coin::Quarter(UsState::Alabama);
    println!("{}",value_in_cents(&coin));
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!",state)
    }else{
        count +=1;
    }
    println!("{}",count);
}
fn route(ip_type:&IpAddr){}

#[derive(Debug)]
enum IpAddr {
    v4(u8,u8,u8,u8),
    v6(String),
}
#[derive(Debug)]
enum Message{
    Quit, // 无关联数据
    Move{x:i32,y:i32}, // 匿名结构体
    Write(String), // string
    ChangeColor(i32,i32,i32),// 3个i32
}

impl Message{
    fn call(&self){
        println!("{:?}",self)
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    // --略--
}
fn value_in_cents(coin:&Coin) -> u32 {
    match coin {
        Coin::Penny =>{
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!",state);
            25
        },
        _ => 0, // 通配符，可以理解成 default
    }
}

fn plus_one(x:Option<i32>)-> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}