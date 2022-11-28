pub fn demo(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}",val);
    }
}

#[test]
fn iterator_demonstration(){
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    // // 暂不清楚下面2个的用法，后续再说。
    // v1.into_iter(); // v1所有权并返回本身的迭代器。
    // v1.iter_mut(); // 可变引用的迭代器

    assert_eq!(v1_iter.next(),Some(&1));
    assert_eq!(v1_iter.next(),Some(&2));
    assert_eq!(v1_iter.next(),Some(&3));
    assert_eq!(v1_iter.next(),None);
}
#[test]
fn iterator_sum(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    // 消耗迭代器
    let total:i32 = v1_iter.sum();

    assert_eq!(total,6);

    //assert_eq!(v1_iter.next(),None); // value borrowed here after move,使用next,必须把迭代器设置从mut

}

#[test]
fn iterator_adaptor(){
    // 迭代适配器
    let v1:Vec<i32> = vec![1,2,3];
    let v2:Vec<_> = v1.iter().map(|x|x+1).collect(); // 用闭包定义了一个适配器做了一次加工，数量相同
    assert_eq!(v2,vec![2,3,4])
}

// 迭代器  增加一个过滤器
#[derive(PartialEq,Debug)]
struct Shoe{
    size:u32,
    style:String,
}
fn shoes_in_my_size(shoes:Vec<Shoe>,shoe_size:u32)->Vec<Shoe>{
    shoes.into_iter()
        .filter(|s|s.size==shoe_size)
        .collect()
}

#[test]
fn filters_by_size(){
    let shoes = vec![
        Shoe{size:10,style:String::from("sneaker")},
        Shoe{size:30,style:String::from("sandal")},
        Shoe{size:10,style:String::from("boot")},
    ];

    let in_my_size = shoes_in_my_size(shoes,10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe{size:10,style:String::from("sneaker")},
            Shoe{size:10,style:String::from("boot")},
        ]
    )
}

/********************************自定义迭代器：实现Iterator trait*************************************/
struct Counter {
    count:u32,
}
impl Counter{
    // 先来个构造器
    fn new() -> Counter{
        Counter{count:0}
    }
}
// 实现某个trait的时候，编译器会帮助咱们拉取定义，咱们自己只要实现todo就好了
impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        }else {
            None
        }
    }
}
#[test]
fn calling_next_directly(){
    let mut counter = Counter::new();

    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);
}
#[test]
fn using_other_iterator_trait_methods(){
    let sum:u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a,b)|a*b)
        .filter(|x|x%3==0)
        .sum();
    assert_eq!(18,sum);
}

/***********************************音频解码*****************************************/
pub fn audio_decoding(){
    let buffer:&mut[i32] = &mut []; // 需要有默认值？？
    let coefficients:[i64;12]= [10,20,30,40,10,20,30,40,10,20,30,40];
    let qlp_shift:i16 = 0; // 需要有默认值？？

    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
            .zip(&buffer[i-12..i])
            .map(|(&c,&s)|c*s as i64)
            .sum::<i64>() >> qlp_shift;
        let delta = buffer[i];
        buffer[i]  = prediction as i32 + delta;
    }

    println!("{:?}",buffer)
}