use std::fmt::Display;

#[derive(Debug)]
struct Point<T1, T2>{
    x: T1,
    y: T2,
}

impl<T1, T2> Point<T1, T2>{
    // <T3, T4> is function template parameter
    fn x<T3, T4>(self, other: Point<T3, T4>) -> Point<T1, T4>{
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

// specification of template
impl Point<f64, f64>{
    fn print_double(self){
        println!("your parameter is float 64! {:#?}", self);
    }
}

// 1. Trait is interface of other programming languages.
// 2. Trait may be as constraint of generic parameter, which is mean that generic
// parameter should be satisfied certain constraints, e.g. T satisfied PartialOrd  and Copy trait (interface).
// fn largest<T:  PartialOrd + Copy >(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// Clone trait
// fn largest<T:  PartialOrd + Clone >(list: &[T]) -> T {
//     let mut largest = list[0].clone();

//     for item in list {
//         // > is included the PartialOrd trait, which is needed the & parameter
//         if item > &mut largest{
//             largest = item.clone();
//         }
//     }
//     largest
// }

// How to implement this method?
// 另一种 largest 的实现方式是返回在 slice 中 T 值的引用。如果我们将函数返回值从 T 改为 &T
// 并改变函数体使其能够返回一个引用，我们将不需要任何 Clone 或 Copy 的 trait bounds
// 而且也不会有任何的堆分配。尝试自己实现这种替代解决方式吧！如果你无法摆脱与生命周期有关的错误,
// 请继续阅读：接下来的 “生命周期与引用有效性” 部分会详细的说明，不过生命周期对于解决这些挑战来说并不是必须的。
// 这里不加生命周期注解也是正确的，但是我认为应该加，这意味着 list 和 返回值能活得一样久
// Display trait 为了打印 largest 使用
fn largest<T:  PartialOrd+ Display>(list: &[T]) -> &T {
    // 实际上下面这种写法被移入到 rust 编译器中，因为 Rust 团队发现在某些特定的情况下，
    // Rust 程序员总是重复地写一模一样的生命周期注解。然而这些场景在 Rust 编程中是可预测的。
    // 这种现象称为生命周期省略
// fn largest<'a, T:  PartialOrd+ Display>(list: &'a [T]) -> &'a T {
    let mut largest = &list[0];
    println!("Largest init value {}", largest);
    for item in list {  // item &T
        // > is included the PartialOrd trait, which is needed the & parameter
        if item > largest{
            largest = item;
        }
    }
    largest  // 这为什么不是悬垂引用？ 因为它实际上指向的是 list
}

pub trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// trait is interface as return value that return unique type object only
fn returns_summarizable(_switch: bool) -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
    // The following code is error because the return value type is not unique.
    // if switch {
    //     NewsArticle {
    //         headline: String::from(
    //             "Penguins win the Stanley Cup Championship!",
    //         ),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best \
    //             hockey team in the NHL.",
    //         ),
    //     }
    // } else {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from(
    //             "of course, as you probably already know, people",
    //         ),
    //         reply: false,
    //         retweet: false,
    //     }
    // }
}


fn main() {
    println!("----generic----");
    let p1 = Point{
        x: 5,
        y: 10.4,
    };
    let p2 = Point{
        x: "hello",
        y: 'c',
    };
    let p3 = p1.x(p2);
    println!("{:#?}", p3);

    println!("----template specification----");
    let pf64: Point<f64, f64> = Point{
        x: 10.4,
        y: 10.5,
    };
    pf64.print_double();
    // p3.print_double(); // method not found in `Point<{integer}, char>`
    println!("----trait----");
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    println!("----trait is interface as return value----");
    // trait is interface as return value
    let result = returns_summarizable(false);
    let strings = result.summarize();
    println!("{}", strings);
}
