use std::process;
use kvdis::{command::Command, connection::{bind, run}, dictionary::{Dictionary}};
use sap::{Parser, Argument};

const DEFAULT_PORT: u16 = 7777;

fn main() {
    let mut parser = Parser::from_env().unwrap();

    let mut port: u16 = DEFAULT_PORT;

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Long("port") => {
                port = parser.value().unwrap().parse().unwrap_or_else(|_e| {
                    eprintln!("Port could not be parsed, reverting to default...");
                    DEFAULT_PORT
                });
            }

            _ => {
                eprintln!("Invalid arguments! Exiting...");
                process::exit(-1);
            }
        }
    }

    let mut dict = Dictionary::new();
    run(&mut dict, &bind(Some(port))).unwrap();
}

fn _cli(dict: &mut Dictionary) {
    loop {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Err(e) => {
                eprintln!("Could not read from stdin: {e}");
                continue;
            },

            Ok(_n_read) => {
                match line.parse::<Command>() {
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                        continue;
                    }

                    Ok(com) => {
                        println!("{}", dict.run(com));
                    }
                };
            }
        };
    }
}
