use std::{io::{self, BufRead, BufReader, BufWriter, Write}, net::TcpListener};

use crate::{command::Command, dictionary::Dictionary};

type Port = u16;
const DEFAULT_PORT_STR: &str = "7777";

pub fn bind(port: Option<Port>) -> TcpListener {
    let listener = match port {
        Some(port) => {
            TcpListener::bind(format!("127.0.0.1:{port}"))
        }
        None => {
            TcpListener::bind(format!("127.0.0.1:{}", DEFAULT_PORT_STR))
        }
    };

    match listener {
        Err(e) => panic!("Listening socket could not be created: {e}"),
        Ok(l) => l
    }
}

/// # Half-duplex connection
/// Commands are received, processed, and responded to.
pub fn run(dict: &mut Dictionary, listener: &TcpListener) -> io::Result<()> {
    for stream in listener.incoming() {
        let stream = stream?;
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        let mut command = String::new();
        reader.read_line(&mut command).unwrap();

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
