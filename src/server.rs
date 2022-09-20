use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use crate::{MyError, SmartSocket};

pub struct ServerHandle {
    listener: TcpListener,
    state: State,
}

struct State {
    socket: SmartSocket,
}

impl ServerHandle {
    pub fn new(addr: impl ToSocketAddrs) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        let state = State {
            socket: SmartSocket::new(),
        };
        Self { listener, state }
    }

    pub fn run(&mut self) -> Result<(), MyError> {
        for stream in self.listener.incoming() {
            ServerHandle::handle_client(stream?, &mut self.state).ok();
        }
        Ok(())
    }

    fn handle_client(mut stream: TcpStream, state: &mut State) -> Result<(), MyError> {
        let mut buf = [0u8; 128];
        loop {
            let read = stream.read(&mut buf)?;
            if read == 0 {
                continue;
            }
            let bytes = &buf[..read];

            if let Ok(s) = String::from_utf8(bytes.to_vec()) {
                let s = s
                    .strip_suffix("\r\n")
                    .or_else(|| s.strip_suffix('\n'))
                    .unwrap_or(&s);

                println!("received string: {}", s);
                let parts = s.split('\n');

                for msg in parts {
                    if msg == "exit" {
                        return Ok(());
                    }

                    match process_message(state, msg) {
                        Ok(resp) => {
                            writeln!(&mut stream, "{}", resp)?;
                        }
                        Err(error_str) => {
                            write!(&mut stream, "{}\r\n", error_str)?;
                        }
                    }
                }
            }
        }
    }
}

fn process_message(state: &mut State, msg: impl AsRef<str>) -> Result<String, &'static str> {
    let msg = msg.as_ref();
    if msg == "status" {
        Ok(state.socket.to_string())
    } else if msg == "turn on" {
        state.socket.turn_on();
        Ok("+OK".to_string())
    } else if msg == "turn off" {
        state.socket.turn_off();
        Ok("+OK".to_string())
    } else if msg.starts_with("load ") {
        let msg = msg
            .strip_prefix("load ")
            .ok_or("error while parsing load")?;
        let load: u32 = msg.parse().map_err(|_| "error while parsing load")?;
        state.socket.set_load(load);
        Ok("+OK".to_string())
    } else {
        Err("-unknown command")
    }
}
