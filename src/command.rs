use std::{time::Duration};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Set(String, String),
    Get(String),
    Del(String),
    Exists(String),
    // TODO: humantime can have more than 1 argument!!!!!!!!!! (ex. 5h 5m 5s)
    Expire(String, Duration),
    Incr(String),
    Decr(String)
}
