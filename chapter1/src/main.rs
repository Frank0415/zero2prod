//fn test();

fn main() {
    println!("Hello, world!");
    let chinese = "111";
    let world = "hello world";
    let region = [chinese, world];
    for reg in region {
        println!("{}", &reg);
    }
    //let mut tp=test();
    split_test();
}

fn _test() -> i32 {
    return 1_i32;
}

fn split_test() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // let _field: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        let temp = record.split(',');

        let mut collector: Vec<&str> = Vec::new();

        for tp in temp {
            collector.push(tp.trim());
        }

        let name = collector[0];
        if let Ok(length) = collector[1].parse::<f32>() {
            println!("{},{}cm", name, length);
        }
    }
}
