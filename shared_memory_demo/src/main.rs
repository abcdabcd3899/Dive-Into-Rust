use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Mutex 相当于 RefCell 这样的智能指针，这个内存需要被多个所有者共享，需要使用 Arc
    // 共享所有权
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // 这里的 clone 只是计算器加 1
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
