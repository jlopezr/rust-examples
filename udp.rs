use std::net::UdpSocket;
 
fn foo() -> std::io::Result<()> {
    let socket = try!(UdpSocket::bind("127.0.0.1:34254"));
    let mut buf = [0; 100];
    let (amt, src) = try!(socket.recv_from(&mut buf));
    let buf = &mut buf[..amt];
    buf.reverse();
    try!(socket.send_to(buf, &src));
    drop(socket); // close the socket
    Ok(())
}

fn main() {
    let x = foo();
    println!("{:?}", x);
}
