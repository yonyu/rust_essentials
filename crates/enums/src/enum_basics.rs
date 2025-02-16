// An enum is a type that can be one of multiple variants
// Each variant can have different types and amounts of associated data
// Enums are most commonly used to create a type that can take on multiple
// values, but they can also be used to create a type that can take on
// multiple types of values

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    route(home);
    route(loopback);
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("V4"),
        IpAddrKind::V6 => println!("V6"),
    }
}
