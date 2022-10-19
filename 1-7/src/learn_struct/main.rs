fn main(){
    let width1 = 50;
    let height1 = 50;

    println!("area={}",area(width1, height1));

    let rect1 = (50,50);
    println!("area1={}",area1(rect1));

    let rect2 = Rectangle{width:50,height:50};
    println!("par={:#?},arae2={}",rect2,area2(&rect2));
    println!("par={:?},arae={}",rect2,rect2.area());

    let ret3 = Rectangle{width:60,height:60};
    let ret4 = Rectangle{width:10,height:40};
    println!("can rect2 hold ret3? {}",rect2.can_hold(&ret3));
    println!("can rect2 hold ret4? {}",rect2.can_hold(&ret4));

    let ret5 = Rectangle::square(45);
    println!("ret5={:?}",ret5);
}

/// 青铜
fn area(width:u32,heigth:u32)-> u32{
    width*heigth
}
/// 白银
/// 元组的操作实现面积计算
fn area1(dimensions:(u32,u32))-> u32{
    dimensions.0*dimensions.1
}

/// 黄金
fn area2(rectangle :&Rectangle)->u32 {
    rectangle.width*rectangle.height
}

#[derive(Debug)] // 添加这一行就可以使用{:?}或{:#?}来进行打印了
struct Rectangle{
    width : u32,
    height : u32,
}

impl Rectangle {
    /// 函数.构造器
    fn square(size:u32)-> Rectangle{
        Rectangle{width:size,height:size}
    }
    /// 方法.面积
    fn area(&self)->u32{
        self.width*self.height
    }
    /// 方法.当前矩形是否包含传入的矩形
    fn can_hold(&self,other : &Rectangle)->bool{
        self.width > other.width && self.height > other.height // 简单的对比是宽对宽，长对长；复杂的是先得到2个矩形的长边和短边，而后长对长，短对短
    }
}
