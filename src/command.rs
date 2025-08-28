use std::{time::Duration};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Set(String, String),
    Get(String),
    Del(String),
    Exists(String),
    Expire(String, Duration),
    Incr(String),
    Decr(String)
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandResult {
    Set,
    Get(String),
    Del,
    Exists(bool),
    Expire,
    Incr,
    Decr
}
