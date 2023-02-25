fn modify1(x: &mut i32){
    *x = 999;
}

fn modify2(ref mut  x: i32){
    *x = 999;
}

fn modify3(ref mut x: &mut i32){
    **x = 999;
}

fn main(){
    println!("----case 1: define variable----");
    let a = 32;
    let b = &a;
    // let c = ref a;  //error
    let ref c = a;
    println!("{} {} {}", a, *b, *c);

    println!("----case 2: define function----");
    let mut a: i32 = 111;
    modify1(&mut a);
    println!("{}", a);  // 999
    let a: i32 = 111;
    modify2(a);
    println!("{}", a); // 111

    let mut a: i32 = 111;
    modify3(&mut a);
    println!("{}", a); // 999
    println!("----case 3: define match----");
    let s = Some(String::from("Hello, World!"));
    match s {
        Some(ref t) => println!("{}", t),
        _ => (),
    }
    // match &s {
    //     Some(t) => println!("{}", t),
    //     _ => (),
    // }
    println!("{:#?}", s);
}