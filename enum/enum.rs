// 枚舉
enum IpAddressKind {
    V4,
    V6
}

struct IpAddress {
    kind: IpAddressKind,
    address: String
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

// 多種類型組合之枚舉
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum 也可以 impl
impl Message {
    fn call(&self) {
        // do something
    }
}

fn main() {
    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1")
    };

    let home_2 = IpAddr::V4(String::from("127.0.0.1"));
    
    println!("{}\n{}\n{:?}", home.address, loopback.address, home_2);
}