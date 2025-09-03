use std::{io::{Read}, net::{TcpListener}};

use crate::{command::Command, dictionary::Dictionary};

// TODO: binding logic for spec ports and error handling
pub fn bind() -> TcpListener {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener
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
