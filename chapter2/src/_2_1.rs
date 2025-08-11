fn _rs2_1_1() {
    let mut a: i32 = 5;
    println!("The val of x is {}", a);
    a = 6;
    println!("The val of x is {}", a);
    let _x: i64 = 10;
}

fn _rs2_1_2() {
    let (_a, mut _b): (bool, i32) = (false, 1);
    println!("a = {}, b = {:?}", _a, _b);
}

#[allow(dead_code)]
struct Struct {
    e: i32,
}

#[allow(dead_code)]
const MAX_PTS: u32 = 100_000;

fn _rs2_1_3() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

fn _rs2_1_4() {
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    let _spaces:&str="      ";

    let _spaces = _spaces.len() as i32;

    {
        // 在当前的括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}