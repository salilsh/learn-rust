#[derive(Debug)]
enum IpAddrKind {
  V4,
  V6,
}

#[derive(Debug)]
struct IpAddr {
  kind: IpAddrKind,
  addr: String, 
}

fn main() {
  let home = IpAddr {
    kind: IpAddrKind::V4,
    addr: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
    kind: IpAddrKind::V6,
    addr: String::from("::1")
  };

  println!("{:?}", home);
  println!("{:?}", loopback);

}
