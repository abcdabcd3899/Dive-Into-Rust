
use std::{collections::HashMap, mem::size_of_val};
fn main() {
    let str = "hello";
    println!("str mem size {}", size_of_val(&str));
    // 长度为 0
    let c1 = || println!("hello world!");
    // 和参数无关，长度也为 0
    let c2 = |i: i32| println!("hello: {}", i);
    let name = String::from("hello,world");
    println!("name mem size {}", size_of_val(&name));
    let name1 = name.clone();
    let mut table = HashMap::new();
    println!("table mem size {}", size_of_val(&table));
    table.insert("hello", "world");
    // 如果捕获一个引用，长度为 8，只包含了一个栈上的指针
    let c3 = || println!("hello: {}", name);
    // 捕获移动的数据 name1(长度 24) + table(长度 48)，closure 长度 72
    let c4 = move || println!("hello: {}, {:?}", name1, table);
    let name2 = name.clone();
    // 和局部变量无关，捕获了一个 String name2，closure 长度 24
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");  // 局部变量，不算闭包的大小
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    )
}

// 闭包的大小只与捕获的变量有关，与局部变量无关。
// 不带 move 时捕获了局部变量的引用，引用就是一根指针，所以大小为 8 (在 64 位系统里)
// 带 move 时获取了变量的所有权，所以对于 String 而言大小是 24 (8+8+8), HashMap 大小是
// 48, 具体数据见上面代码