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

/// Treats everything as a String,
/// `incr` and `decr` operations work on parsable i64 bound Strings
impl Dictionary {
    pub fn new() -> Self {
        Dictionary { map: HashMap::new() }
    }

    /// Runs a `Command`
    /// ### Since all the error checking is done in parsing time, commands should never fail
    /// # Returns 
    /// The command result wrapped in `command::CommandResult`
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
            Incr(key) => {
                self.incr(&key);
                Some(CommandResult::Incr)
            },
            Decr(key) => {
                self.decr(&key);
                Some(CommandResult::Decr)
            }
        }
    }

    pub fn set(&mut self, key: String, value: Entry) {
        self.map.insert(key, value);
    }

    /// Gets the value mapped to `key` returns `Some(String)`
    /// If not found or expired, returns `None`
    pub fn get(&self, key: &str) -> Option<String> {
        match self.map.get(key) {
            Some(e) => {
                // Check if expired
                if let Some(lifetime) = e.expiration {
                    if lifetime <= SystemTime::now() {
                        return None;
                    }
                }

                Some(e.value.clone())
            },
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
                entry.expiration = Some(SystemTime::now() + lifetime);
            },
            // TODO
            None => {}
        }
    }

    // NOTE: use custom functions instead of working on the map directly

    pub fn incr(&mut self, key: &str) {
        // TODO yeah... Best look for a way to handle this
        let old_val = self.get(&key).unwrap().parse::<i64>().unwrap();
        let new_val = old_val + 1;
        self.set(key.to_string(), Entry {
            value: new_val.to_string(),
            expiration: None
        });
    }
    pub fn decr(&mut self, key: &str) {
        // TODO yeah... Best look for a way to handle this
        let old_val = self.get(&key).unwrap().parse::<i64>().unwrap();
        let new_val = old_val - 1;
        self.set(key.to_string(), Entry {
            value: new_val.to_string(),
            expiration: None
        });
    }
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

    #[test]
    fn get_set_expired() {
        let mut dict = Dictionary::new();

        let set_command = "SET metanoia 19".to_string().parse::<Command>().unwrap();
        // expire in 1 second
        let expire_command = "EXPIRE metanoia 1s".parse::<Command>().unwrap();
        let get_command = "GET metanoia".to_string().parse::<Command>().unwrap();

        dict.run(set_command);
        dict.run(expire_command);

        // Not expired yet; should be Some
        let got = dict.run(get_command);
        assert_eq!(got, Some(CommandResult::Get("19".to_string())));

        // sleep for 2 seconds
        std::thread::sleep(Duration::from_secs(2));

        // Expired; should be None
        let get_command = "GET metanoia".to_string().parse::<Command>().unwrap();
        let got = dict.run(get_command);
        assert_eq!(got, None);
    }

    #[test]
    fn incr() {
        let mut dict = Dictionary::new();

        let set_command = "SET something 5".parse::<Command>().unwrap();
        let incr_command = "INCR something".parse::<Command>().unwrap();

        dict.run(set_command);

        // Check that it is indeed that command
        assert_eq!(dict.run(incr_command), Some(CommandResult::Incr));

        // Check that it worked (5+1 = 6)
        let get_command = "GET something".parse::<Command>().unwrap();
        assert_eq!(dict.run(get_command), Some(CommandResult::Get("6".to_string())));
    }

    #[test]
    fn decr() {
        let mut dict = Dictionary::new();

        let set_command = "SET something -5".parse::<Command>().unwrap();
        let decr_command = "DECR something".parse::<Command>().unwrap();

        dict.run(set_command);

        // Check that it is indeed that command
        assert_eq!(dict.run(decr_command), Some(CommandResult::Decr));

        // Check that it worked (-5-1 = -6)
        let get_command = "GET something".parse::<Command>().unwrap();
        assert_eq!(dict.run(get_command), Some(CommandResult::Get("-6".to_string())));
    }
}
