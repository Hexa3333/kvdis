use std::{fs, time::{Duration, SystemTime}};

use kvdis::{command::{Command, CommandResult}, connection::{accept, bind}, dictionary::{Dictionary, Entry}, errors::{DictionaryError, ParseError}};
use sap::{Parser, Argument};

const DEFAULT_PORT: u16 = 1453;

fn main() {
    let mut parser = Parser::from_env().unwrap();

    let mut _port: u16;

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Long("port") => {
                _port = parser.value().unwrap().parse().unwrap_or_else(|_e| {
                    eprintln!("Port could not be parsed, reverting to default...");
                    DEFAULT_PORT
                });
            }

            _ => panic!("Invalid argument!")
        }
    }

    let mut dict = Dictionary::new();

    dict.set(String::from("1"), Entry {
        value: String::from("bir"),
        expiration: None
    });
    dict.set(String::from("2"), Entry {
        value: String::from("iki"),
        expiration: None
    });
    dict.set(String::from("3"), Entry {
        value: String::from("üç"),
        expiration: Some(SystemTime::now() + Duration::from_secs(5))
    });

    dict.expire("3", Duration::from_secs(3));

    let set_command = "SET something whatevs".parse::<Command>().unwrap();
    let get_command = "GET something".parse::<Command>().unwrap();
    dict.run(set_command).unwrap();
    dict.run(get_command).unwrap();

    let html = dict.dump_html();
    fs::write("./dump.html", html).unwrap();

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
                        match e {
                            ParseError::NotACommand => {
                                eprintln!("Not a command!");
                            },
                            ParseError::InvalidParameters => {
                                eprintln!("Command parameters are invalid!");
                            },
                            ParseError::IsEmpty => {}
                        }

                        continue;
                    }

                    Ok(com) => {
                        match dict.run(com) {
                            Err(e) => {
                                match e {
                                    DictionaryError::DoesNotExist => {
                                        eprintln!("Key does not exist.");
                                    },
                                    DictionaryError::IsExpired => {
                                        eprintln!("Key has expired.");
                                    }
                                };
                                continue;
                            }

                            Ok(ret) => {
                                match ret {
                                    CommandResult::Get(got) => {
                                        println!("{got}");
                                    },
                                    CommandResult::Exists(check) => {
                                        println!("{check}");
                                    }

                                    _ => {}
                                }
                            }
                        };
                    }
                };
            }
        };
    }
}
