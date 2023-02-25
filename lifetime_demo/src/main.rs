// 所有使用了引用的函数，都需要生命周期的标注
// 生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的 出自《The Rust Programming Book》

pub fn strtok<'a, 'b>(s: &'b mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // 由于 delimiter 可以是 utf8，所以我们需要获得其 utf8 长度，
        // 直接使用 len 返回的是字节长度，会有问题
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else { // 如果没找到，返回整个字符串，把原字符串指针 s 指向空串
        let prefix = *s;
        *s = "";
        prefix
    }
}
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(&string1, string2);
    println!("The longest string is {}", result);

    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 这里实际上相当于做了这么几件事情：
    /*
        let x: &str = "xxx";
        let y: &str = "yyy";
        let ret: &str = &x; or let ret: &str = &y, 我们无法知道 &x 和 &y 的生命周期，
        如果它们存活的不一样久，比如另外一个函数在某次调用中 move 了 x 或者 y，那么这里的
        调用将会出现无效的引用，悬挂起来
        在这里我们看到，我们无法知道最终 ret 到底是引用 x或 y
        比如，test1() 和 test2() 函数所示，生命周期不确定，生命周期关联性未知
    */
    if x.len() > y.len() {
        x
    }else{
        y
    }
}

fn _test1(){
    let x = 5;
    let y = &x;
    println!("y {}", y);
}