pub fn for_loops() {
    let mut arr = [1, 2, 3, 4, 5];
    let mut arr2: [i32; 5] = std::array::from_fn(|i| (i * i) as i32);
    let mut arr3: [String; 5] = std::array::from_fn(|_| String::from("value"));

    for i in &mut arr3 {}

    for (i, v) in arr2.iter().enumerate() {
        println!("This is the {}th element, {}", i, v);
    }

    for _ in 0..10 {}

    for _ in 0..=10 {}
}
