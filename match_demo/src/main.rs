fn main() {
    // let pattern
    let i: Option<i32> = Some(21);
    if let Some(res) = i {
        println!("{}", res);
    }else {
        println!("None!");
    }

    // it is correct, but it is not need.
    let x = 30;
    if let y = x {
        println!("{}", y);
    }

    let (.., y) = (1, 2);
    println!("..y is {}", y);

    // it uses the match pattern in Closure
    let c = |(x, y): (i32, i32)|{
        println!("{} {}", x, y);
    };
    c((1, 2));

    // while let pattern
    let mut v = vec![1, 2, 3];
    v.push(4);
    while let Some(res) = v.pop(){
        println!("{:#?}", res);
    }

    // for pattern
    v.push(1);
    v.push(2);
    v.push(3);
    for (index, value) in v.iter().enumerate(){
        println!("{}, {}", index, value);
    }
}
