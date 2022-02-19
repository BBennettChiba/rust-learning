use std::cell::RefCell;
use std::rc::{Rc, Weak};

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             List::Cons(_, item) => Some(item),
//             _ => None,
//         }
//     }
// }

// use List::{Cons, Nil};

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
//     println!("an initial rc count is {}", Rc::strong_count(&a));
//     println!("a next item is {:?}", a.tail());
//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("an rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count =  {}", Rc::strong_count(&b));
//     println!("b next item is {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

//     // println!("a = {:?}", a);
// }

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {} weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
        
        println!(
            "leaf strong = {} weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
        println!(
            "branch strong = {} weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
    }

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {} weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
