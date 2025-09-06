use std::{collections::HashMap, time::{Duration, SystemTime}, sync::{Arc, Mutex}};

use crate::{command::{Command, CommandResult}, errors::DictionaryError};

#[derive(Debug)]
pub struct Entry {
    pub value: String,
    pub expiration: Option<SystemTime>
}

#[derive(Debug)]
pub struct Dictionary {
    pub map: Arc<Mutex<HashMap<String, Entry>>>
}

/// Treats everything as a String,
/// `incr` and `decr` operations work on parsable i64 bound Strings
impl Dictionary {
    pub fn new() -> Self {
        Dictionary { map: Arc::new(Mutex::new(HashMap::new())) }
    }

    /// Runs a `Command`
    /// ### Since all the error checking is done in parsing time, commands should never fail
    /// # Returns 
    /// The command result wrapped in `command::CommandResult`
    pub fn run_headless(&mut self, command: Command) -> Result<CommandResult, DictionaryError> {
        use Command::*;
        match command {
            Set(key, value) => {
                self.set(key, Entry { value, expiration: None });
                Ok(CommandResult::Set)
            },
            Get(key) => {
                return Ok(CommandResult::Get(self.get(&key)?));
            },
            Del(key) => {
                self.del(&key);
                Ok(CommandResult::Del)
            },
            Exists(key) => {
                Ok(CommandResult::Exists(self.exists(&key)))
            },
            Expire(key, lifetime) => {
                self.expire(&key, lifetime);
                Ok(CommandResult::Expire)
            },
            Incr(key) => {
                self.incr(&key);
                Ok(CommandResult::Incr)
            },
            Decr(key) => {
                self.decr(&key);
                Ok(CommandResult::Decr)
            }
        }
    }

    pub fn run(&mut self, command: Command) -> String {
        match self.run_headless(command) {
            Err(e) => e.to_string(),
            Ok(ret) => ret.to_string()
        }
    }

    pub fn set(&mut self, key: String, value: Entry) {
        let mut map = self.map.lock().unwrap();
        map.insert(key, value);
    }

    /// # Returns
    /// - If found, Ok(String)
    /// - If expired, Err(CommandError::IsExpired)
    /// - If it does not exist, Err(CommandError::DoesNotExist)
    pub fn get(&self, key: &str) -> Result<String, DictionaryError> {
        let map = self.map.lock().unwrap();
        match map.get(key) {
            Some(e) => {
                // Check if expired
                if let Some(lifetime) = e.expiration {
                    if lifetime <= SystemTime::now() {
                        return Err(DictionaryError::IsExpired);
                    }
                }

                Ok(e.value.clone())
            },
            None => Err(DictionaryError::DoesNotExist)
        }
    }

    pub fn del(&mut self, key: &str) {
        let mut map = self.map.lock().unwrap();
        map.remove(key);
    }

    pub fn exists(&self, key: &str) -> bool {
        let map = self.map.lock().unwrap();
        match map.get(key) {
            None => false,
            Some(value) => match value.expiration {
                None => true,
                Some(expiration) => {
                    if SystemTime::now() <= expiration {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    pub fn expire(&mut self, key: &str, lifetime: Duration) {
        let mut map = self.map.lock().unwrap();
        match map.get_mut(key) {
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

        assert_eq!(dict.run_headless(set_command), Ok(CommandResult::Set));
        let got = dict.run_headless(get_command);
        assert_eq!(got, Ok(CommandResult::Get("19".to_string())));
    }

    #[test]
    fn get_set_expired() {
        let mut dict = Dictionary::new();

        let set_command = "SET metanoia 19".to_string().parse::<Command>().unwrap();
        // expire in 1 second
        let expire_command = "EXPIRE metanoia 1s".parse::<Command>().unwrap();
        let get_command = "GET metanoia".to_string().parse::<Command>().unwrap();

        dict.run_headless(set_command).unwrap();
        dict.run_headless(expire_command).unwrap();

        // Not expired yet; should be Some
        let got = dict.run_headless(get_command);
        assert_eq!(got, Ok(CommandResult::Get("19".to_string())));

        // sleep for 2 seconds
        std::thread::sleep(Duration::from_secs(2));

        // Expired; should be None
        let get_command = "GET metanoia".to_string().parse::<Command>().unwrap();
        let got = dict.run_headless(get_command);
        assert_eq!(got, Err(DictionaryError::IsExpired));
    }

    #[test]
    fn get_doesnt_exist() {
        let mut dict = Dictionary::new();

        let get_command = "GET metanoia".to_string().parse::<Command>().unwrap();
        let got = dict.run_headless(get_command);
        assert_eq!(got, Err(DictionaryError::DoesNotExist));
    }

    #[test]
    fn incr() {
        let mut dict = Dictionary::new();

        let set_command = "SET something 5".parse::<Command>().unwrap();
        let incr_command = "INCR something".parse::<Command>().unwrap();

        dict.run_headless(set_command).unwrap();

        // Check that it is indeed that command
        assert_eq!(dict.run_headless(incr_command), Ok(CommandResult::Incr));

        // Check that it worked (5+1 = 6)
        let get_command = "GET something".parse::<Command>().unwrap();
        assert_eq!(dict.run_headless(get_command), Ok(CommandResult::Get("6".to_string())));
    }

    #[test]
    fn decr() {
        let mut dict = Dictionary::new();

        let set_command = "SET something -5".parse::<Command>().unwrap();
        let decr_command = "DECR something".parse::<Command>().unwrap();

        dict.run_headless(set_command).unwrap();

        // Check that it is indeed that command
        assert_eq!(dict.run_headless(decr_command), Ok(CommandResult::Decr));

        // Check that it worked (-5-1 = -6)
        let get_command = "GET something".parse::<Command>().unwrap();
        assert_eq!(dict.run_headless(get_command), Ok(CommandResult::Get("-6".to_string())));
    }
}
