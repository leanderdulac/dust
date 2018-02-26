use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("api.pagar.me:80").unwrap();
    let mut x = [0; 128];

    let req = String::from("GET /1/status\n\n");
    let req_bytes = req.into_bytes();
    let w = stream.write(&req_bytes);
    let r = stream.read(&mut x);

    println!("w{:?}", w);
    println!("r{:?}", r);
    println!("{:?}", String::from_utf8_lossy(&x));
}
