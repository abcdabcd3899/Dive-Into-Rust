extern "C" {
    fn triple_input(input: i32)-> i32;
}
fn unsafe_pointer(){
    let mut a = 10;
    let b = &a as *const i32;
    let c = &mut a as *mut i32;  // 同时存在可变指针 c 和 不可变指针 b
    println!("{:#?}", b);
    unsafe {
        (*c) += 1;
        println!("{:#?}", *b);  //  解引用必须发生在 unsafe block 中
        println!("{:#?}", *c);
    };
}

unsafe fn unsafe_function(){
    println!("----unsafe function or method----");
    let mut a = 10;
    let b = &a as *const i32;
    let c = &mut a as *mut i32;  // 同时存在可变指针 c 和 不可变指针 b
    println!("{:#?}", b);
    (*c) += 1;
    println!("{:#?}", *b);  //  解引用必须发生在 unsafe block 中
    println!("{:#?}", *c);  // 在 unsafe function 中操作 unsafe code 无需增加 unsafe block
}

static mut COUNT: i32 = 0;  // 谨慎使用全局变量，这在多线程程序中可能造成 data race

fn main() {
    unsafe_pointer();
    unsafe{
        unsafe_function();  // 需要 unsafe block
        let c = triple_input(3);
        println!("{}", c);
    }
    unsafe{
        COUNT += 3;
        println!("COUNT is {}", COUNT);
    }
}


// 1. 创建一个裸指针不会有任务危险，只有当解引用这个指针时才有可能触发危险，因此，解引用裸指针
// 必须要在 unsafe 代码块中
// rust FFI 2. https://github.com/alexcrichton/rust-ffi-examples
// https://stevenbai.top/rustbook/book/interoperability/c-with-rust.html