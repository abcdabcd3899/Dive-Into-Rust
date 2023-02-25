use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn split_first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    // How to get the type of variable?
    // 1. Using the following method assigns any types to variable.
    // Rust compiler output the correct type for us.
    // let bytes: i32 = s.as_bytes();
    // 2. enable rust-analyzer to prompt the user
    for (i, &item) in bytes.iter().enumerate(){
        // if item == ' ' as u8{
        // if item == b' '{
        if item as libc::c_char == (' ' as i8).try_into().unwrap(){
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("Hello World!");
    println!("{}", split_first_word(&s));
    // String::clear(&mut s);
    // s.clear(); // the error is same as above

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number {}", secret_number);
    // loop{
    //     println!("Please input your guess: ");
    //     let mut guess = String::new();

    //     io::stdin().read_line(&mut guess).expect("Failed to read line");
    //     let guess: u32 = match guess.trim().parse(){
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     match guess.cmp(&secret_number){
    //         Ordering::Less => println!("Too small!!"),
    //         Ordering::Greater => println!("Too big!!"),
    //         Ordering::Equal => {
    //             println!("You win!!");
    //             break;
    //         }
    //     };
    // }

    println!("----Iterator----");
    let arr = vec![1,2,3];
    let mut arr_iter = arr.into_iter();
    println!("{:?}", arr_iter.next());
    println!("------Borrow------");
    let data = vec![1, 2, 3, 4];
    let data1 = &data;

    println!(
        "addr of value (heap): {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );

    // let mut data = vec![1, 2, 3];
    // for item in data.iter_mut(){
    //     data.push(*item + 1);
    // }

    // 下面写法是错误的
    // let mut arr = vec![1, 2, 3];
    // // cache the last item
    // let last = arr.last();  // 这里 arr 是不可变借用
    // arr.push(4);  // 这里 arr 是可变借用
    // // consume previously stored last item
    // println!("last: {:?}", last);

    // 下面是对的
    let mut arr = vec![1, 2, 3];
    // cache the last item
    arr.push(4);  // 这里 arr 是可变借用
    let last = arr.last();  // 这里 arr 是不可变借用
    // consume previously stored last item
    println!("last: {:?}", last);

    // 或者。顺序很重要，这
    let mut arr = vec![1, 2, 3];
    // cache the last item
    let last = arr.last();  // 这里 arr 是不可变借用
    // consume previously stored last item
    println!("last: {:?}", last);
    arr.push(4);  // 这里 arr 是可变借用
}


fn sum(data: &Vec<u32>) -> u32 {

    println!("addr of value (heap): {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}

// unit test for every methods
#[cfg(test)]
mod test_split{
    use super::*;

    #[test]
    fn test_split_word(){
        let s = String::from("hello worldwide");
        let ret = split_first_word(&s);
        assert_eq!(ret, "hello");
        let s = " ";
        let ret = split_first_word(s);
        assert_eq!(ret, "");

        let s = "";
        let ret = split_first_word(s);
        assert_eq!(ret, "");
    }

    #[test]
    fn test_split_word_single(){
        let s = "hello";
        let ret = split_first_word(s);
        assert_eq!(ret, "hello");
    }
}