use std::{io::{Read}, net::{TcpListener}};

use crate::{command::Command, dictionary::Dictionary};

type Port = u16;
const DEFAULT_PORT_STR: &str = "7777";

// TODO: binding logic for spec ports and error handling
pub fn bind(port: Option<Port>) -> TcpListener {
    match port {
        Some(port) => {
            TcpListener::bind(format!("127.0.0.1:{port}")).unwrap()
        }
        None => {
            TcpListener::bind(format!("127.0.0.1:{}", DEFAULT_PORT_STR)).unwrap()
        }
    }
}

pub fn accept(dict: &mut Dictionary, listener: &TcpListener) {
    for stream in listener.incoming() {
        let mut command = String::new();
        stream.unwrap().read_to_string(&mut command).unwrap();

        // TODO: Error handling (external to module)
        let command = command.parse::<Command>().unwrap();
        dict.run(command).unwrap();
    }
}
