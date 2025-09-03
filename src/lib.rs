//! kvdis
//! A relational map of Strings
//! - Still haven't decided how text GET and EXISTS calls should process or do whatever.

pub mod command;
pub mod parsing;
pub mod errors;
pub mod dictionary;
pub mod connection;
pub mod persistence;
