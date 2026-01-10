// enums allows to enumerate a list of variants
// used when a value can be one of several different types
// each variant can have different types and amounts of associated data

enum IpAddrKind{
    V4(u8, u8, u8, u8),
    // V4(String),
    V6(String)
}
enum Message{
    Quit,
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}
fn main() {
    println!("Hello, world!");
    

    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    // let localhost = IpAddrKind::V4(String::from("127.0.0.1"));

// ----------------------------------------------------------

    // let localhost = IpAddr{
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };
// ----------------------------------------------------------
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
}

fn route(ip_kind: IpAddrKind){

}