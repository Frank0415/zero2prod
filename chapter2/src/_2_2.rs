fn _rs2_2_3() {
    let (mut a, _c) = (32, false);
    let mut _b: Vec<i32> = Vec::new();
    let y: i32 = {
        a = a * 2;
        a += 1;
        a
    };
    let _z = if y % 2 == 1 {
        1;
    } else {
        2;
    };
}

fn _rs2_2_4_1() {
    let _a = 10;
}

fn _dead_end() -> ! { // -> !表示永不返回的函数
  panic!("你已经到了穷途末路，崩溃吧！");
}

