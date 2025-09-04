enum Option<T> {
    None,
    Some(T),
}



#[derive(Debug)]
enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ipV6: IPAddrKind = IPAddrKind::V4(1,3,4,5);
    let loopback  : IPAddrKind = IPAddrKind::V6(String::from("::1"));

    route(ipV6);
    route(loopback);
}

fn route(ip: IPAddrKind){
    println!("Routing request to {:?}", ip);
}

//? Another way with struct method and enum

// #[derive(Debug)]
// enum IPAddrKind {
//     V4, 
//     V6,
// }

// struct IpAddress {
//     address: String,
//     kind: IPAddrKind,
// }

// impl IpAddress{
//     fn new(address: &str) -> Self{
//         Self {
//             address : address.to_string(),
//             kind  : IPAddrKind::V4
//         }
//     }
// }

// fn main() {
//     let google_address = IpAddress::new("1.3.4.5");
//     let loopback = IpAddress::new("::1"); //127.0.0.1

//     route(loopback);
// }

// fn route(ip: IpAddress) {
//     println!("Routing request to IP {} of kind {:?}", ip.address, ip.kind);
// }