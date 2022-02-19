// use std::{thread, time::Duration};
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     handle.join().unwrap();
//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     // handle.join().unwrap();
// }

// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     handle.join().unwrap();
// }

use std::sync::mpsc;
use std::thread;
fn main(){
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let msgs = vec![String::from("hello"), String::from("from"), String::from("the"), String::from("thread")];
        for msg in msgs.into_iter() {
            tx.send(msg).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
        // tx.send(msg).unwrap();
        // println!("send msg: {}", msg);
    });

    thread::spawn(move || {
        let msgs = vec![String::from("more"), String::from("messages"), String::from("for"), String::from("you")];
        for msg in msgs.into_iter() {
            tx2.send(msg).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    for received in rx {
        println!("Got: {}", received);
    }
    
}