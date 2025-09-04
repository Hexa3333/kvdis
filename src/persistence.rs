use crate::dictionary::{Dictionary, Entry};
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
mod savingdata {
    use std::time::{SystemTime, Duration};

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
}
