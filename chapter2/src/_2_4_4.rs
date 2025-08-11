#[allow(unused)]
enum Pokercards {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

enum Pokercards_improve {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

struct Ipv4Addr;
struct Ipv6Addr;

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
