use std::{
    io::{self, Read, Write},
    mem,
    net::TcpStream,
    thread,
};

use smarthome::server::ServerHandle;

fn main() -> Result<(), io::Error> {
    let addr = "127.0.0.1:7878";
    let mut s = ServerHandle::new(addr);
    let sh = thread::spawn(move || s.run());

    let mut stream = TcpStream::connect(addr)?;
    let mut buf = [0u8; 128];

    let command_seq = [
        "status", "turn on", "load 100", "status", "turn off", "status", "exit",
    ];

    for cmd in command_seq {
        write!(stream, "{}\r\n", cmd)?;
        let resp = read_line(&mut buf, &mut stream)?;
        println!("response: {}", resp);
    }

    // stream.write(b"status\n")?;

    // let resp = read_line(&mut buf, &mut stream)?;
    // println!("response: {}", resp);

    // stream.write(b"exit\n")?;
    // stream.read(&mut buf)?;
    mem::drop(stream);

    _ = sh.join();
    println!("exiting");
    Ok(())
}

fn read_line(buf: &mut [u8], r: &mut impl Read) -> Result<String, io::Error> {
    let len = r.read(buf)?;
    Ok(String::from_utf8(buf[0..len].to_vec()).unwrap())
}
