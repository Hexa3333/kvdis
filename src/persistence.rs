use crate::{dictionary::{Dictionary, Entry}, errors::SerializationError};
use std::{collections::HashMap, fs, io, path::PathBuf, sync::{Arc, Mutex}};

const STORAGE_PATH: &str = "./db.csv";
pub struct Serializer {
    map: Arc<Mutex<HashMap<String, Entry>>>,
    path: PathBuf
}

#[allow(dead_code)]
impl Serializer {
    pub fn new(dict: &Dictionary) -> Self {
        Serializer {
            map: Arc::clone(&dict.map),
            path: PathBuf::from(STORAGE_PATH)
        }
    }

    pub fn load_file_csv(&mut self) -> Result<(), SerializationError> {
        let csv = fs::read_to_string(&self.path).unwrap();
        self.set_from_csv(&csv)
    }

    pub fn save_file_csv(&self) -> io::Result<()> {
        let csv = self.get_as_csv();
        fs::write(&self.path, &csv)
    }

    pub fn set_from_csv(&mut self, csv: &str) -> Result<(), SerializationError> {
        // NOTE: possible poisoning
        let mut guard = self.map.lock().unwrap();
        guard.clear();
        drop(guard);

        for line in csv.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            let key = parts.get(0).ok_or(SerializationError::KeyRead)?;
            let value = parts.get(1).ok_or(SerializationError::ValueRead)?;
            let expiration = match parts.get(2) {
                Some(exp) => {
                    match exp.parse::<humantime::Timestamp>() {
                        Ok(exp) => Some(exp),
                        Err(_e) => {
                            return Err(SerializationError::TimestampRead);
                        }
                    }
                },
                None => None
            };

            // NOTE: possible poisoning
            let mut guard = self.map.lock().unwrap();
            guard.insert(key.to_string(), Entry {
                value: value.to_string(), 
                expiration: match expiration {
                    Some(exp) => Some(exp.into()),
                    None => None
                }
            });
        }

        Ok(())
    }

    /// Locks the map and creates a csv String from it
    pub fn get_as_csv(&self) -> String {
        let map = self.map.lock().unwrap();

        let mut s = String::new();
        for (key, entry) in map.iter() {
            let mut line = String::new();
            line.push_str(key);
            line.push(',');
            line.push_str(&entry.value);
            match entry.expiration {
                Some(exp) => {
                    line.push(',');
                    let exp = humantime::format_rfc3339(exp).to_string();
                    line.push_str(&exp);
                },
                None => {}
            }

            s.push_str(&line);
            s.push('\n');
        }

        s
    }
}

#[cfg(test)]
mod persistence {
    use std::{time::{Duration, SystemTime}};

    use crate::errors::DictionaryError;

    use super::*;

    #[test]
    fn csv_str() {
        let mut dict = Dictionary::new();
        dict.set("enjoy".to_string(), Entry {
            value: "yourself".to_string(),
            expiration: None
        });

        let time = SystemTime::now() + Duration::from_secs(15);
        dict.set("liar".to_string(), Entry {
            value: "pants_on_fire".to_string(),
            expiration: Some(time)
        });
        let s = Serializer::new(&dict).get_as_csv();

        let time_str = humantime::format_rfc3339(time).to_string();
        assert!(s.contains("enjoy,yourself\n"));
        assert!(s.contains("liar,pants_on_fire,{}\n"
                .to_string().replace("{}", &time_str).as_str()));
    }

    #[test]
    fn csv_to_map() {
        let dict = Dictionary::new();
        let csv = "1,one\n2,two\n3,three";

        let mut sd = Serializer::new(&dict);
        sd.set_from_csv(&csv).unwrap();

        assert_eq!(dict.get("1").unwrap(), "one".to_string());
        assert_eq!(dict.get("2").unwrap(), "two".to_string());
        assert_eq!(dict.get("3").unwrap(), "three".to_string());
    }

    #[test]
    fn csv_to_map_with_expiration_valid() {
        let dict = Dictionary::new();

        // NOTE: January 1st 2100, 12 o'clock
        let csv = "1,one\n2,two\n3,three,2100-01-01T00:00:00Z";

        let mut sd = Serializer::new(&dict);
        sd.set_from_csv(&csv).unwrap();

        assert_eq!(dict.get("1").unwrap(), "one".to_string());
        assert_eq!(dict.get("2").unwrap(), "two".to_string());
        assert_eq!(dict.get("3").unwrap(), "three".to_string());
        assert_eq!(dict.exists("3").to_string(), "true");
    }

    #[test]
    fn csv_to_map_with_expiration_invalid() {
        let dict = Dictionary::new();

        // NOTE: January 1st 2001, 12 o'clock
        let csv = "1,one\n2,two\n3,three,2001-01-01T00:00:00Z";

        let mut sd = Serializer::new(&dict);
        sd.set_from_csv(&csv).unwrap();

        assert_eq!(dict.get("1").unwrap(), "one".to_string());
        assert_eq!(dict.get("2").unwrap(), "two".to_string());
        assert_eq!(dict.get("3"), Err(DictionaryError::IsExpired));
        assert_eq!(dict.exists("3").to_string(), "false");
    }

    #[test]
    fn csv_to_map_expiration_corrupted() {
        let dict = Dictionary::new();

        // NOTE: Corrupted date
        let csv = "1,one\n2,two\n3,three,01-91-01T70:00:00Z";

        let mut sd = Serializer::new(&dict);
        assert_eq!(sd.set_from_csv(&csv), Err(SerializationError::TimestampRead));
    }
}
