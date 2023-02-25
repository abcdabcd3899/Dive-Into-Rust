use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Welcome"),
            String::from("to"),
            String::from("the"),
            String::from("Rust!"),
        ];
        for val in vals{
            tx.send(val).unwrap();
            // println!("val is {}", val); // compile error
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("Rust"),
            String::from("is"),
            String::from("the"),
            String::from("Most"),
            String::from("Interesting"),
            String::from("Languages!"),
        ];
        for val in vals{
            tx_clone.send(val).unwrap();
        }
    });
    for recevied in rx{
        println!("{:#?}", recevied);
    }
}

// 基于 message passing 的并发使用了 channel，channel 本身使用单所有权机制。也就是说
// 一旦将一个值传入到 channel 中，将无法再使用这个值 (上面程序证明了这一点)。但是在很多情况下，
// 我们需要让某块内存拥有多个所有者，使得多个线程能够同时访问同一个内存位置。有两种解决办法:
// 1. 使用智能指针包裹数据，这增加了编程的复杂性
// 2. 基于 shared memory 实现并发，同时给出了许多安全的智能指针支撑
