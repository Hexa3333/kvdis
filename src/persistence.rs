use crate::{dictionary::{Dictionary, Entry}, errors::SerializationError};
use std::{collections::HashMap, sync::{Arc, Mutex}};

pub struct SavingData {
    map: Arc<Mutex<HashMap<String, Entry>>>
}

#[allow(dead_code)]
impl SavingData {
    pub fn new(dict: &Dictionary) -> Self {
        SavingData {
            map: Arc::clone(&dict.map)
        }
    }

    pub fn set_from_csv(&mut self, csv: &str) -> Result<(), SerializationError> {
        for line in csv.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            let key = parts.get(0).ok_or(SerializationError::Key)?;
            let value = parts.get(1).ok_or(SerializationError::Value)?;
            let _expiration = match parts.get(2) {
                Some(exp) => {
                    // TODO: error checking
                    Some(exp.parse::<humantime::Timestamp>().unwrap())
                },
                None => None
            };

            // NOTE: possible poisoning
            let mut guard = self.map.lock().unwrap();
            guard.insert(key.to_string(), Entry { value: value.to_string(), expiration: None });
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
        let s = SavingData::new(&dict).get_as_csv();

        let time_str = humantime::format_rfc3339(time).to_string();
        assert!(s.contains("enjoy,yourself\n"));
        assert!(s.contains("liar,pants_on_fire,{}\n"
                .to_string().replace("{}", &time_str).as_str()));
    }

    #[test]
    fn csv_to_map() {
        let dict = Dictionary::new();
        let csv = "1,one\n2,two\n3,three";

        let mut sd = SavingData::new(&dict);
        sd.set_from_csv(&csv).unwrap();

        assert_eq!(dict.get("1").unwrap(), "one".to_string());
        assert_eq!(dict.get("2").unwrap(), "two".to_string());
        assert_eq!(dict.get("3").unwrap(), "three".to_string());
    }

    #[test]
    fn csv_to_map_with_expiration() {
    }
}
