use std::{collections::HashMap, time::{SystemTime, Duration}};

use crate::command::{Command, CommandResult};

#[derive(Debug)]
pub struct Entry {
    pub value: String,
    pub expiration: Option<SystemTime>
}

#[derive(Debug)]
pub struct Dictionary {
    map: HashMap<String, Entry>
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary { map: HashMap::new() }
    }

    /// ### Since all the error checking is done in parsing time, commands should never fail
    /// ## Returns the command result wrapped in `command::CommandResult`
    pub fn run(&mut self, command: Command) -> Option<CommandResult> {
        use Command::*;
        match command {
            Set(key, value) => {
                self.set(key, Entry { value, expiration: None });
                Some(CommandResult::Set)
            },
            Get(key) => {
                Some(CommandResult::Get(self.get(&key)?))
            },
            Del(key) => {
                self.del(&key);
                Some(CommandResult::Del)
            },
            Exists(key) => {
                Some(CommandResult::Exists(self.exists(&key)))
            },
            Expire(key, lifetime) => {
                self.expire(&key, lifetime);
                Some(CommandResult::Expire)
            },
            Incr(_key) => {
                todo!();
                //Some(CommandResult::Incr)
            },
            Decr(_key) => {
                todo!();
                //Some(CommandResult::Decr)
            }
        }
    }

    pub fn set(&mut self, key: String, value: Entry) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.map.get(key) {
            Some(e) => Some(e.value.clone()),
            None => None
        }
    }

    pub fn del(&mut self, key: &str) {
        self.map.remove(key);
    }

    pub fn exists(&self, key: &str) -> bool {
        self.map.get(key).is_some()
    }

    pub fn expire(&mut self, key: &str, lifetime: Duration) {
        match self.map.get_mut(key) {
            Some(entry) => {
                entry.expiration = Some(SystemTime::now() + lifetime)
            },
            // TODO
            None => {}
        }
    }

    //pub fn incr(&mut self, key: &str);
    //pub fn decr(&mut self, key: &str);
}

#[cfg(test)]
mod commands {
    use super::*;

    #[test]
    fn get_set() {
        let mut dict = Dictionary::new();

        let set_command = "SET metanoia 19".to_string().parse::<Command>().unwrap();
        let get_command = "GET metanoia".to_string().parse::<Command>().unwrap();

        assert_eq!(dict.run(set_command), Some(CommandResult::Set));
        let got = dict.run(get_command);
        assert_eq!(got, Some(CommandResult::Get("19".to_string())));
    }
}
