fn _rs2_4_1_1_sub(name: String) {
    println!("Hello! I am {}", name);
}

fn _rs2_4_1_1_bor(name: &str) {
    println!("Hello! I borrowed {}", name);
}

fn _rs2_4_1_1() {
    let name: String = String::from("Frank");
    _rs2_4_1_1_sub(name);
    let name: String = String::from("Frank");
    let mut subname = &name[..3];
    _rs2_4_1_1_bor(subname);
    subname = &name[1..2];
    _rs2_4_1_1_bor(subname);
}

fn _rs2_4_1_2() {
    let mut s: String = String::from("hello world");

    let word = first_word(&mut s).to_string(); // make an owned String

    s.clear(); // now allowed, as word owns its data

    //let _word2: &str = first_word_u(&s);

    println!("the first word is: {}", word);
}

#[allow(dead_code)]
fn first_word(s: &mut String) -> &str {
    &s[..1]
}

#[allow(dead_code)]
fn first_word_u(s: &String) -> &str {
    &s[..1]
}

/**/

fn _rs2_4_1_3() {
    for c in "中国人".chars() {
        println!("{}", c);
    }
}

pub fn _rs2_4_1_total() {
    let mut str: String = String::from("Hello");
    str.push(' ');
    println!("{}", str);
    str.push_str("rust");
    println!("{}", str);
    str.insert(5, ',');
    println!("{}", str);
    str.insert_str(11, "!");
    println!("{}", str);
    str.pop();
    let _str_: String = String::from(" And");
    str = str + &_str_;
    println!("{}", str);
    let sstr: &str = "1";
    let ssstr: &str = "4";
    str = str + " " + sstr + sstr + ssstr;
    println!("{}", str);
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    str = str + "-" + &s2 + "-" + &s3;
    println!("{}", str);
}

#[allow(unused)]
pub fn _rs2_4_2() {
    let (x, y, z) = (500, 9.1, false);
    let mut tuple = (200, 7.8, true);
    tuple.2 = z as bool;
}

pub fn _rs2_4_5() {
    let arr = [3; 5];

    let array = [
        String::from("rust is good!"),
        String::from("rust is good!"),
        String::from("rust is good!"),
    ];

    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    /*
    array::from_fn 是 Rust 1.63 及以上版本提供的一个函数，用于通过一个生成函数批量初始化固定长度数组。

    基本用法

    let arr: [T; N] = std::array::from_fn(|i| { /* 返回 T */ });
    T：数组元素类型
    N：数组长度
    闭包参数 i：元素索引（从 0 到 N-1）
    例子

    let squares: [u32; 5] = std::array::from_fn(|i| (i * i) as u32);// squares = [0, 1, 4, 9, 16]
    你的代码：


    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    生成了一个长度为 8 的数组，每个元素都是新建的 "rust is good!" 字符串。
    优点
    类型安全：返回的就是固定长度数组 [T; N]，不是 Vec。
    避免手动写循环，代码更简洁。
    每个元素都可以根据索引自定义生成。
    注意事项
    适用于数组长度已知且较小的场景。
    每次调用闭包都会生成一个新元素（如 String），不会复用。
    */
}
