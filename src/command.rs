use std::{time::Duration};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Set(String, String),
    Get(String),
    Del(String),
    Exists(String),
    Expire(String, Duration),
    Incr(String),
    Decr(String),
    Save
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandResult {
    Set,
    Get(String),
    Del,
    Exists(bool),
    Expire,
    Incr,
    Decr,
    Save
}

impl ToString for CommandResult {
    fn to_string(&self) -> String {
        match self {
            CommandResult::Get(got) => {
                got.to_string()
            },
            CommandResult::Exists(check) => {
                check.to_string()
            }

            _ => {
                "".to_string()
            }
        }
    }
}
