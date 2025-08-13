/*
Pattern Matching
*/

#[allow(dead_code)]
enum Direction {
    South,
    North,
    East,
    West,
}

pub fn _matching_easy() {
    let dir_m = Direction::South;
    let _i: i32 = _matching_match(&dir_m);
    let _ii: i32 = _matching_if(&dir_m);
}

pub fn _matching_option(i: i32) {
    let _op_i: Option<i32> = Some(i);
}

fn _matching_match(dir: &Direction) -> i32 {
    let ret: i32 = match dir {
        Direction::South => 1,
        _ => 0,
    };
    match dir {
        Direction::East => println!("East!"),
        _ => (),
    };
    ret
}

fn _matching_if(dir: &Direction) -> i32 {
    let mut ret: i32 = 0;
    if let Direction::East = dir {
        ret = 1;
    };
    ret
}

fn _plus_one(i: Option<i32>) -> Option<i32> {
    let ret: Option<i32> = match i {
        None => None,
        Some(i) => Some(i + 1),
    };
    ret
}

pub fn _matching_misc(i: Option<i32>) -> i32 {
    let Some(val) = i else {
        panic!("i32 is none and cannot be matched.");
    };
    if val == 5 {
        println!("The input is {}", val);
    }
    let mut _vec_i: Vec<Option<i32>> = [Some(1), Some(2), None, Some(3)].to_vec();
    while let Some(p) = _vec_i.pop() {
        if let Some(v) = p {
            println!("{}", v);
        }
    }

    val
}

#[allow(dead_code)]
pub enum Pty {
    Pt3d(i32, i32, i32),
    Pt2d(i32, i32),
    Pt(i32),
    None,
}

pub fn _match_pty(pt: Pty) {
    match pt {
        Pty::Pt3d(.., z) if z != 0 => println!("The z dim is {}", z),
        Pty::Pt2d(x, y) => println!("Only has x {}, y{}", x, y),
        _ => println!("Nothing hi-dim matched"),
    };
}

pub fn _match_misc() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;

    assert_eq!(x, 114);
    assert_eq!(y, 514);

    let arr: &[u16] = &[114, 514];

    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }

    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }
}

pub fn _match_misc_2() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
