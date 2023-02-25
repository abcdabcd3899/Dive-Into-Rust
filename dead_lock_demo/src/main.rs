use std::sync::Mutex;   // 下面程序存在 deadlock，使用该包检测不到
// use no_deadlocks::Mutex; // 使用这个 mutex 将会检测到 deadlock，程序会在运行时 panic
use std::sync::Arc;
use std::{thread, time::Duration};

fn main() {
    let m1 = Arc::new(Mutex::new(5));
    let m2 = Arc::new(Mutex::new(6));

    // 我们现在启动两个定义两个线程，分别以相反的方向获取两个 critical section
    let m3 = Arc::clone(&m1);
    let m4 = Arc::clone(&m2);
    let t1 = thread::spawn(move ||{
        let mut d1 = m3.lock().unwrap();
        *d1+=1;
        let mut d2 = m4.lock().unwrap();
        *d2+=1;
        thread::sleep(Duration::from_millis(10));
    });

    let m5 = Arc::clone(&m1);
    let m6 = Arc::clone(&m2);
    let t2 = thread::spawn(move ||{
        let mut d2 = m6.lock().unwrap();
        *d2+=2;
        let mut d1 = m5.lock().unwrap();
        *d1+=2;
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("m1 {:#?}", *m1.lock().unwrap());
    println!("m2 {:#?}", *m2.lock().unwrap());
}


// 上述情况会造成死锁，因为我们不知道线程 t1 和 t2 将会增样被调度，在某些情况下，他们可能
// 会存在资源访问的交叉关系，导致产生死锁，上面产生死锁的原因是：每个线程都分开获取需要的锁。
// 解决办法是使用 https://crates.io/search?q=deadlock 这样的工具在线程开启时一次性获取
// 需要的锁资源，如果不能拿到全部锁，就会阻塞直到能够获取到所有的资源，这是避免死锁的方法之一。
// 同样我们能够使用 https://crates.io/crates/cooptex 这样的 mutex drop 掉造成死锁的地方
// 使得线程能够正常推进

