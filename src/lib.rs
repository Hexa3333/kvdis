//! kvdis
//! A relational map of Strings
//! - Still haven't decided how text GET and EXISTS calls should process or do whatever.
//! # Command set
//! #### SET \<key\> \<value\>
//! #### GET \<key\>
//! #### DEL \<key\>
//! #### EXISTS \<key\>
//! #### EXPIRE \<key\> \<duration in humantime format\>
//! #### INCR \<key\>
//! #### DECR \<key\>

pub mod command;
pub mod parsing;
pub mod errors;
pub mod dictionary;
pub mod connection;
pub mod persistence;
