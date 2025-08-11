fn _rs2_2_1_1() {
    // 使用 wrapping_add，溢出时会循环到最小值
    let a: u8 = 250;
    let b: u8 = 10;
    let wrapping = a.wrapping_add(b); // 250 + 10 = 260, 超过 u8 最大值 255，结果为 4
    println!("wrapping_add: {}", wrapping);

    // 使用 checked_add，溢出时返回 None
    let checked = a.checked_add(b); // 250 + 10 = 260，溢出，返回 None
    println!("checked_add: {:?}", checked);

    // 使用 overflowing_add，返回元组 (结果, 是否溢出)
    let (overflowing, is_overflow) = a.overflowing_add(b); // 结果为 4，溢出为 true
    println!(
        "overflowing_add: {}, overflowed: {}",
        overflowing, is_overflow
    );

    // 使用 saturating_add，溢出时结果为最大值
    let saturating = a.saturating_add(b); // 结果为 255
    println!("saturating_add: {}", saturating);
}

fn _rs2_2_1_2() {
    // PartialEq is implemented for floats, but Eq is not.
    // This is because floating-point numbers (f32, f64) can be NaN,
    // and NaN is not equal to itself: (NaN != NaN).
    let x = std::f64::NAN;
    println!("NaN == NaN: {}", x == x); // false

    // This violates the reflexivity property required by Eq.
    // Therefore, floats implement PartialEq, but not Eq.
    // Also, there is 0.1+0.2!=0.3
}

fn _rs2_2_1_3() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 != xyz.2);
}

fn _rs2_2_1_4() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }
    let _y: i64 = 16 as i64;
}

fn _rs2_2_1_5() {
    for i in 'a'..='f' {
        println!("{}", i);
    }
}

use num::complex::Complex;

fn _rs2_2_1_6() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}
