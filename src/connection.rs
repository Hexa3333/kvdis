use std::{io::{self, BufRead, BufReader, BufWriter, Write}, net::TcpListener};

use crate::{command::Command, dictionary::Dictionary};

type Port = u16;
pub const DEFAULT_PORT: Port = 7777;

pub fn bind(port: Option<Port>) -> TcpListener {
    let listener = match port {
        Some(port) => {
            TcpListener::bind(format!("127.0.0.1:{port}"))
        }
        None => {
            TcpListener::bind(format!("127.0.0.1:{}", DEFAULT_PORT.to_string()))
        }
    };

    match listener {
        Err(e) => panic!("Listening socket could not be created: {e}"),
        Ok(l) => l
    }
}

/// # Half-duplex connection
/// Commands are received, processed, and responded to.
/// With the exception of SAVE/LOAD calls which run on a seperate thread.
pub fn run(dict: &mut Dictionary, listener: &TcpListener) -> io::Result<()> {
    for stream in listener.incoming() {
        let stream = stream?;
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        let mut command = String::new();
        reader.read_line(&mut command)?;

        let command = match command.parse::<Command>() {
            Ok(com) => com,
            Err(e) => {
                writer.write_all(format!(
                        "[Error]: {}", e.to_string()
                        )
                    .as_bytes())?;
                continue;
            }
        };

        let result = dict.run(command);
        writer.write_all(result.as_bytes())?;
    }

    Ok(())
}
