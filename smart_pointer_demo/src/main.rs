use std::rc::Rc;
use crate::List::{Cons, Nil};
use crate::ListRc::{ConsRc, NilRc};
#[derive(Debug)]
pub enum List<T>{
    Cons(T, Box<List<T>>),
    Nil,
}

#[derive(Debug)]
pub enum ListRc<T>{
    ConsRc(T, Rc<ListRc<T>>),
    NilRc,
}

fn main(){
    let a =  Box::new(5);
    let b = Box::new(6);
    // heap address and stack address
    // 0xaaab04decad0 0xffffd1812ad0
    // 0xaaab04decaf0 0xffffd1812ad8
    // 实际上不需要使用 *a 就能打印引用的值，因为
    // Box 类型实现了 Deref trait
    println!("{:p} {:p} {}", a, &a, *a);
    println!("{:p} {:p} {}" , b, &b, *b);
    println!("----recursive type----");
    //  1 -> 2 -> 3 -> nil
    let list1 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:#?}", list1);
    // a -> b -> c -> nil
    let list2 = Cons("a", Box::new(Cons("b", Box::new(Cons("c", Box::new(Nil))))));
    println!("{:#?}", list2);

    // 我们现在想用上述结构创建下述列表
    // l1 = 5 -> 10 -> nil
    // 3 -> l1
    // 4 -> l1
    // let l1 = Cons(5, Box::new(Cons(10,  Box::new(Nil))));
    // let l2 = Cons(3, Box::new(l1));
    // let l3 = Cons(4, Box::new(l1));  // value used here after move
    let l1 = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    let l2 = ConsRc(3, Rc::clone(&l1));
    let l3 = ConsRc(4, Rc::clone(&l1));
    println!("{:#?}", l2);
    println!("{:#?}", l3);
}