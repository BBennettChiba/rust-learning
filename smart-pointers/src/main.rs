use std::ops::Deref;
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {  
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    print!("b = {}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))));
    let  x=  5;
    // let y = &x;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    // assert_eq!(y, 5);
    assert_eq!(*y, 5);
}
