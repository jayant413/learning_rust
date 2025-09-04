#[derive(Debug)]
enum IPAddrKind {
    V4, 
    V6,
}

struct IpAddress {
    address: String,
    kind: IPAddrKind,
}

impl IpAddress{
    fn new(address: &str) -> Self{
        Self {
            address : address.to_string(),
            kind  : IPAddrKind::V4
        }
    }
}

fn main() {
    let google_address = IpAddress::new("1.3.4.5");
    let loopback = IpAddress::new("::1"); //127.0.0.1

    route(loopback);
}

fn route(ip: IpAddress) {
    println!("Routing request to IP {} of kind {:?}", ip.address, ip.kind);
}