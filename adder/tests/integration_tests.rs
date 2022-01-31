use adder::double;
mod common;

#[test]
fn test1(){
    assert_eq!(4, double(2));
    common::hello();
}
