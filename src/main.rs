use std::io;
use std::net::TcpStream;
use owo_colors::OwoColorize;
use ssh2::Session;
use std::io::prelude::*;

fn main() {

    println!("Input your desire IP address to connect:");

    let mut ip = String::new();

    io::stdin()
        .read_line(&mut ip)
        .expect("Failed to read line");

    println!("Input your desire port to connect:");

    let mut port = String::new();

    io::stdin()
        .read_line(&mut port)
        .expect("Failed to read line");

    let tcp = match TcpStream::connect(format!("{}:{}", ip.trim(), port.trim())){
        Ok(tcp) => tcp,
        Err(err) => {
            eprintln!("Error connecting to {}:{}", ip.trim().blue(), port.trim().blue());
            eprintln!("Error details: {}", err);
            return;
        }
    };

    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("username", "password")
        .unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.exec("show version").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().ok();
    println!("{}", channel.exit_status().unwrap());
}
