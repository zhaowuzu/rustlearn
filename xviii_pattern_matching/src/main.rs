fn main() {
    // ？？何为模式
    // 匹配类型结构的语法。   数据形状==需要匹配的值

    // match 分支
    // 1. 必须穷尽
    // 2. 可以使用 _模式 来匹配所以的可能的值

    // if let 也是顺序匹配
    // 无需穷尽
    let favorite_color:Option<&str> = None;
    let is_tuesday = false;
    let age:Result<u8,_> = "34".parse();

    if let Some(color) = favorite_color{
        println!("Using your favorite color,{},as the background",color);
    }else if is_tuesday {
        println!("Tuesday is green day");
    }else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        }else {
            println!("Using orange as the backage color");
        }
    }else {
        println!("Using blue as the background color");
    }

    // while let 条件循环
    // 会反复执行，直到出现失败的情形
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}",top);
    }

    // for 循环
    let v = vec!['a','b','c'];
    for (index,value) in v.iter().enumerate() { // enumerate 适配器来返回对应的元组
        println!("{} is at index {}",value,index)
    }

    // let 直接
    let x = 5;
    let (x,y,z) = (1,2,3); // (1,2,3)与模式（x,y,z）是一一对应的
    // let (x,y) = (1,2,3);// 匹配不成功
    let (x,..,_) = (1,2,3);// .. 和 _ 忽略对应匹配

    // 函数的参数，函数的参数也是模式
    let point = (3,5);
    print_coordinates(&point);

    // 模式：不可失败，可失败 两种
    let x = 5;// 不可失败
    let some_option_value : Option<i32> = Some(1);
    // 函数参数，let,for只可接收不可失败模式
    if let zz = 5 { // 其实这里没有任何意义。因为它不会失败。irrefutable pattern
        println!("{}",x)
    };
    // if let,while let 表达式只接收可失败模式
    // let Some(x) = some_option_value;// type annotations needed for `Option<T>` pattern `None` not covered
    if let Some(x) = some_option_value { // 把let换成if let 就ok了
        println!("{}",x);
    };

    // # 模式匹配字面量
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // # 模式匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y)=> println!("Matched,y= {:?}",y),
        _ => println!("Default case,x = {:?}",x),
    }
    println!("at the end:x={:?},y={:?}",x,y);// y并没有在match中被改变

    // # 多重模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // # 使用..=来匹配值区间
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a' ..= 'j' => println!("early ASCII letter"),
        'k' ..= 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // # 解构结构体
    let p = Point{x:0,y:7};
    // 解构
    let Point{x:a,y:b} = p; // 用模式匹配结构体，且是不一样的名字
    let Point{x,y} = p;     // 用模式匹配结构体，一样的名字的简写法
    assert_eq!(0,a);
    assert_eq!(7,b);
    assert_eq!(0,x);
    assert_eq!(7,y);
    // 匹配
    match p {
        Point {  x,y:0}=> println!("On the x axis at {}",x), // x轴上的点
        Point{x:0,y}=> println!("On the y axis at {}",y), // y轴上的点
        Point{x,y}=> println!("On neither axis: ({},{})",x,y),// 不在坐标轴上的点
    }

    // # 结构枚举
    // 注意：用于解构枚举的模式必须要对应枚举定义中存储数据的方式
    //let msg = Message::ChangeColor(0,160,255);
    let msg = Message::ColorEnum(Color::Hsv(0,160,255));
    match msg {
        Message::Quit =>{
            println!("The Quit variant has no date to destructure.");
        },
        Message::Move {x,y} => {
            println!("Move in the x direction {} add in the y direction {}",x,y);
        },
        Message::Write(text) => println!("Text message: {}",text),
        Message::ChangeColor(r,g,b) => {
            println!("Change the color to red {},green {},and blue {}",r,g,b);
        },
        Message::ColorEnum(Color::Rgb(r,g,b)) => {
            println!("Change the color to red {},green {},and blue {}",r,g,b);
        },
        Message::ColorEnum(Color::Hsv(h,s,v)) => {
            println!("Change the color to hue {},saturation {},and value {}",h,s,v);
        },
    }

    // # 混合解构
    let ((feet,inches),Point{x,y}) = ((3,10),Point{x:3,y:-10});
    println!("feet={},inches={},x={},y={}",feet,inches,x,y);

    // # _忽略值
    foo1(3,4);
    // 业务不允许用户覆盖某个设置中已经存在的自定义选项，但它允许用户重置选项并在选项未初始化时进行设置
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value,new_setting_value) {
        // 不允许覆盖
        (Some(_),Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        // 允许初始化
        _ => {
            setting_value = new_setting_value
        }
    }
    println!("setting is {:?}",setting_value);
    // 让后多次使用_忽略多个特定的值
    let numbers = (2,4,8,16,32);
    match numbers {
        (first,_,third,_,fifth) => {
            println!("Some numbers:{},{},{}",fifth,third,fifth);
        }
    }
    // 使用_忽略未使用的变量
    // _x 仍然会继续绑定到x变量上，而_则完全不会进行绑定
    let _x = 5;
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s{ // 虽说使用了_,但是仍然绑定到了_s,所有权发生了转移
        println!("found a string");
    }
    //println!("{:?}",s);// 这里会报错
    let s = Some(String::from("Hello!"));
    if let Some(_) = s { // 没有进行绑定操作，不会发生转移
        println!("found a string");
    }
    println!("{:?}",s);// 这里不会报错
    // 使用..模式忽略值的剩余部分
    let origin = Point1{x:0,y:0,z:0};
    match origin {
        Point1{x,..} => println!("x is {}",x),
    }
    let numbers = (2,4,8,16,32);
    match numbers {
        (first,..,last) => {
           println!("Some numbers:{},{}",first,last);
        }
    }
    // match numbers {
    //     (..,second,..) => { // 会报错，不确定使用的是中间的哪个值 `..` can only be used once per tuple pattern
    //         println!("Some numbers:{}",second);
    //     }
    // }

    // # 使用匹配守卫添加额外的条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five:{}",x),
        Some(x) => println!("{}",x),
        None => (),
    }
    // 使用匹配守卫来测试some变体内的值是否与外部变量相等
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Match,n = {:?}",n),// if n== y 不是一个模式，所以不会引入新的变量，就会直接使用外部的y
        _ => println!("Default case,x={:?}",x),
    }
    println!("at the end:x={:?},y={:?}",x,y);
    // 将匹配守卫与多重模式组合使用
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), // 优先级关系是 (4 | 5 | 6) if y
        _ => println!("no"),
    }
    // 使用@运算符把值绑定到变量中
    let msg = Message1::Hello {id:5};
    match msg {
        Message1::Hello {id:id_variable @ 3..=7} => { // 使用@  会把匹配成功的值赋值给id_variable。
            println!("Found an id in range:{}",id_variable);
        },
        Message1::Hello {id:10..=12}=>{// 没有使用@  虽然会匹配成功，但不知道是10到12直接的哪个值
            println!("Found an id another range");
        },
        Message1::Hello {id}=>{
            println!("Found some other id:{}",id);
        }
    }
}

// 函数参数也是模式
fn foo(x:i32){}// x部分就是模式
fn print_coordinates(&(x,y):&(i32,i32)) {
    println!("Current location:({},{})",x,y);
}

// # 解构结构体
struct Point {
    x:i32,
    y:i32,
}

// # 解构枚举,嵌套
enum Color{
    Rgb(i32,i32,i32),
    Hsv(i32,i32,i32),
}
enum Message {
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
    ColorEnum(Color),
}

// # 使用_忽略值
// 首先是函数
fn foo1(_:i32,y:i32){
    println!("This code only uses the y parameter:{}",y);
}
//使用..模式忽略值的剩余部分
struct Point1{
    x:i32,
    y:i32,
    z:i32,
}

// 使用@运算符把值绑定到变量中
enum Message1{
    Hello {id:i32}
}