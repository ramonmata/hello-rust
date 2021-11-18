#![allow(unused)]

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        // TODO: Call something here...
        println!("Calling something...{:?}", self);
    }
}


fn main() {
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    println!("{:?}, {:?}", ipv4, ipv6);

    route(&ipv4);
    route(&ipv6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    println!("{:?}, {:?}", home, loopback);

    let m = Message::Move {
        x: 1, y: 3
    };

    m.call();

    let mut x:Option<String> = None;
    let y = Some(10);
    
    println!("{:?}", x);
    println!("{:?}", y);

    x.replace(String::from("Hi"));
    
    let z = Some(1);

    let a = z.zip(x).zip(y);
    println!("A: {:?}", a);
    
}

fn route(ip: &IpAddrKind) {
    println!("Route to {:#?}", ip);
}