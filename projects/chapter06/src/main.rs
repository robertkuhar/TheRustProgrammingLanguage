
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
    println!("v4: {:?}, v6: {:?}", v4, v6);


}
