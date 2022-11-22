use adder;

mod common;

#[test]
fn it_addr_two(){
    common::setup();
    assert_eq!(4,adder::add(2,2))
}