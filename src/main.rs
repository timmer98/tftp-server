use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::io::prelude;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:7878").unwrap();

    loop {
        handle_connection(&socket);
    }
}

fn handle_connection(socket: &UdpSocket) {
    let mut buffer = [0; 1024];
    let (amount, src) = socket.recv_from(&mut buffer).unwrap();
    println!("Received {} from {}", amount, src);

    socket.send_to(&buffer[..amount], src).unwrap();
}
