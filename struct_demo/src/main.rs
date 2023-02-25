#[derive(Debug)]
struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct UnitStruct;

impl User{
    // method
    pub fn get_user_name(&self) -> &String{
        return &self.username;
    }
    // association function
    pub fn create(_active: bool, _email: String, _username: String, _sign_in_count: u64) ->Self{
        return Self{
            active: _active,
            email: _email,
            username: _username,
            sign_in_count: _sign_in_count,
        }
    }
}


struct Color(i32, i32, i32);

fn main() {
    let user1 = User{
        active: true,
        email: String::from("zhangsan@email.xxx"),
        username: String::from("zhangsan"),
        sign_in_count: 1,
    };
    let user2 = User{
        active: user1.active,
        email: String::from("lisi@email.xxx"),
        username: String::from("lisi"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{} {} {} {}", user1.active, user1.email,
                            user1.username, user1.sign_in_count);
    println!("{} {} {} {}", user2.active, user2.email,
                            user2.username, user2.sign_in_count);
    let user3 = User{
        email: String::from("wangwu@email.xxx"),
        ..user1
    };
    println!("{} {} {} {}", user3.active, user3.email,
                            user3.username, user3.sign_in_count);
    // error: due user.email typename is String, which is moved
    // println!("{} {} {} {}", user1.active, user1.email,
    //                         user1.username, user1.sign_in_count);
    let user4 = User{
        email: dbg!(user2.email.clone()),
        active: user2.active,
        username: user2.username.clone(),
        sign_in_count: user2.sign_in_count,
    };
    dbg!(&user4);
    //  Yes, it can use clone method instead of moving value
    println!("{} {} {} {}", user4.active, user4.email,
                            user4.username, user4.sign_in_count);
    let user5 = user4;
    println!("{} {} {} {}", user5.active, user5.email,
                            user5.username, user5.sign_in_count);
    println!("{:#?}", user5);
    println!("user5.username is: {}", user5.get_user_name());
    // error: user4 is moved
    // println!("{} {} {} {}", user4.active, user4.email,
    //                         user4.username, user4.sign_in_count);

    // tuple struct
    let mut c = Color(0, 0, 0);
    println!("{} {} {}", c.0, c.1, c.2);
    c.0 = 255;
    println!("{} {} {}", c.0, c.1, c.2);

    // tuple
    let mut t = (0, 1, 2);
    println!("{} {} {}", t.0, t.1, t.2);
    t.0 = 255;
    println!("{} {} {}", t.0, t.1, t.2);

    let user6 = User::create(false, String::from("xxx"),
                            String::from("xxx"), 10);
    dbg!(&user6);

    println!("{}", "----unit struct test demo---");
    let u = UnitStruct;
    println!("{:#?}", u);
}