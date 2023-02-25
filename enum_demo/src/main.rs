use std::any::type_name;  // type_of
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::io::{Error, ErrorKind};

fn type_of<T: ?Sized>(_: &T) -> &'static str {
    type_name::<T>()
}

#[derive(Debug)]
struct Ip{
    v4: (u8,u8, u8, u8),
    v6: String,
}

#[derive(Debug)]
enum Coin{
    One,
    Two,
    Ten,
}

//  It is the same as struct Ip for the functions.
#[derive(Debug)]
enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("{}", "-------Struct-------");
    let home = Ip{
        v4: (127, 0, 0, 1),
        v6: String::from("::1"),
    };
    println!("{} {} {} {}", home.v4.0, home.v4.1, home.v4.2, home.v4.3);
    println!("{:#?}", home);
    println!("{}", home.v6);
    dbg!(&home);

    println!("{}", "-------Enumeration-------");

    let home_enum = IpAddr::V4(127, 0, 0, 1);
    println!("{:#?}", home_enum);
    let office = IpAddr::V6(String::from("::1"));
    println!("{:#?}", office);

    println!("{}", "-------Option Enumeration-------");
    let some_number_1 = Some::<i32>(5);
    let some_number_2 = Some(5);
    let some_number_3: Option<i32> = Some(5);
    println!("{:#?}", some_number_1);
    println!("{:#?}", some_number_2);
    println!("{:#?}", some_number_3);
    let null_number_1 = Option::<i32>::None;
    let null_number_2: Option<i32> = None;
    println!("{:#?}", null_number_1);
    println!("{:#?}", null_number_2);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // https://doc.rust-lang.org/std/option/index.html
    let sum = x + y.expect("panic");
    println!("Sum = {}", sum);

    println!("{}", "-------Match and Option Enumeration-------");
    let z = match y {
        None => None,
        Some(i) => Some(i + x),
    };
    println!("z typename is {}", type_of(&z));
    println!("{:#?}", z);

    let coin = Coin::Ten;
    let c = match coin{
        Coin::One => {
            println!("One");
            1
        }
        Coin::Two =>{
            println!("Two");
            2
        }
        Coin::Ten => {
            println!("Ten");
            10
        }
    };
    println!("c is {:#?}", c);

    let d = 9;
    let dd: i32 = match d {
        3 => {
            println!("Three");
            3
        }
        7 => {
            println!("Seven");
            4
        }
        other => {
            println!("{}", other);
            other
        }
    };
    println!("{}", d);
    println!("{}", dd);

    println!("----Collections----");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);

    let result = read_username_from_file();
    println!("{:#?}", result);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
