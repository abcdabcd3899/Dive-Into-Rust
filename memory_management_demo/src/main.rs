use std::mem::{align_of, size_of};
use std::collections::HashMap;
// #[repr(C)] // 取消编译器优化
struct S1 {
    a: u8,
    b: u16,
    c: u8,
}


struct S2 {
    a: u8,
    c: u8,
    b: u16,
}

enum Color{
    A(u8),
    B(u8),
    C(String),
    D(HashMap<i8, i8>),  // 48
}

fn main() {
    println!("sizeof S1: {}, S2: {}", size_of::<S1>(), size_of::<S2>());
    println!("alignof S1: {}, S2: {}", align_of::<S1>(), align_of::<S2>());
    let c = size_of::<Color>();
    println!("sizeof Color: {}", c);
    println!("sizeof Char {}", size_of::<char>());  // 4 byte unicode character
}


// 结构体中的内存对齐，这是 C 语言中的对齐方式，在 Rust 中也遵循这样的方式，但是 Rust 会重排 struct 中的元素位置
// 1. 首先是对齐单位，任何一个程序中的对齐单位都是 #progma pack(number)中指定的数字 a 与struct 中的最长数据类型 b 中较小的那个.u = min{a, b}
// 2. 其次是每个数据类型的起始地址都是该元素数据类型大小 b 与对齐单位 u 中小的那个值的整数倍 start = min{u, b}
// 3. 最后是整个 struct 的大小必须是对齐单位 u 的整数倍
// 更多内存信息参见 https://cheats.rs/#standard-library-types