pub trait Messenger {
    fn send(&self,msg: &str);
}

pub struct LimitTracker<'a,T:'a + Messenger> {
    messenger:&'a T,
    value:usize,
    max:usize,
}

impl <'a,T> LimitTracker<'a,T>
    where T:Messenger {
    pub fn new(messenger:&T,max:usize) ->LimitTracker<T>{
        LimitTracker{
            messenger,
            value:0,
            max,
        }
    }

    pub fn set_value(&mut self,value:usize){
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error:You are over your quota!")
        }else if percentage_of_max>= 0.9 {
            self.messenger.send("Urgent warning:You've used up over 90% of your quota!");
        }else if percentage_of_max>= 0.75 {
            self.messenger.send("Warning:You've used up over 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::cell::RefCell;

    struct  MockMessenger {
        //sent_messages:Vec<String>,// 未使用RefCell前
        sent_messages:RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new()-> MockMessenger {
            // 未使用RefCell前
            //MockMessenger{sent_messages:vec![]}

            MockMessenger{sent_messages:RefCell::new(vec![])}
        }
    }

    impl Messenger for MockMessenger{
        fn send(& self,message:&str){
            // 未使用RefCell前
            //self.sent_messages.push(String::from(message));
            self.sent_messages.borrow_mut().push(String::from(message));// borrow_mut 可变引用 mut=mutation变化

            // // 延时一个错误的使用，同一作用域创建2个可变借用，导致的pannic
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();
            //
            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message(){
        let  mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger,100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(),1); // borrow 不可变引用
    }
}