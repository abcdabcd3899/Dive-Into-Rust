use std::sync::{Arc, Condvar};
use no_deadlocks::Mutex;
use std::thread;

fn main() {
    let cond1 = Arc::new((Mutex::new(true), Condvar::new()));
    let cond2 = Arc::new((Mutex::new(false), Condvar::new()));
    let d1 = Arc::clone(&cond1);
    let d2 = Arc::clone(&cond2);
    let t1 = thread::spawn(move||{
        let (lock1, cvar1) = &*d1;
        let mut data1 = lock1.lock().unwrap();
        *data1 = false;
        cvar1.notify_one();
        let (lock2, cvar2) = &*d2;
        let mut data2 = lock2.lock().unwrap();
        *data2 = true;
        cvar2.notify_one();

    });

    let d3 = Arc::clone(&cond1);
    let d4 = Arc::clone(&cond2);
    let t2 = thread::spawn(move||{
        let (lock1, cvar1) = &*d4;
        let data1 = lock1.lock().unwrap();
        let(lock2, cvar2) = &*d3;
        let data2 = lock2.lock().unwrap();
    });

    t1.join().unwrap();
    t2.join().unwrap();
}


// 需要注意的是 Condition Variable 和 Mutex 必须成对出现
// 使用条件变量导致的死锁