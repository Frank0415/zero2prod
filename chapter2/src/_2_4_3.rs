#[allow(unused)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
}

#[allow(unused)]
#[derive(Debug)]
struct Point(f64, f64, f64);

#[allow(unused)]
pub fn single_user() {
    let mut user1: User = User {
        active: true,
        username: String::from("Frank"),
        email: String::from("frank20050415@sjtu"),
    };
    let pt = Point(0 as f64, 0 as f64, 0 as f64);
    println!("{:?} located at {:?}", user1, pt);
}

#[allow(unused)]
fn give_user(username: String, email: String) -> User {
    let ret_user: User = User {
        active: true,
        username,
        email,
    };

    ret_user
}
