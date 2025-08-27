use std::{str::FromStr, time::Duration};

use crate::errors::ParseError;

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

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();

        use Command::*;
        match words[0] {
            "SET" => {
                if words.len() != 3 {
                    return Err(ParseError::InvalidParameters);
                } else {
                    Ok(Set(words[1].to_string(), words[2].to_string()))
                }
            },
            "GET" => {
                if words.len() != 2 {
                    return Err(ParseError::InvalidParameters);
                } else {
                    Ok(Get(words[1].to_string()))
                }
            },
            "DEL" => {
                if words.len() != 2 {
                    return Err(ParseError::InvalidParameters);
                } else {
                    Ok(Del(words[1].to_string()))
                }
            }

            "EXISTS" => {
                if words.len() != 2 {
                    return Err(ParseError::InvalidParameters);
                } else {
                    Ok(Exists(words[1].to_string()))
                }
            }

            "EXPIRE" => {
                if words.len() != 3 {
                    return Err(ParseError::InvalidParameters);
                } else {
                    Ok(Expire(words[1].to_string(), words[2].parse::<humantime::Duration>().map_err(|_e| {
                        ParseError::InvalidParameters
                    })?.into()))
                }
            }

            _ => Err(ParseError::NotACommand)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expire() {
        let s = "EXPIRE alex 3s";
        let com = s.parse::<Command>();

        assert_eq!(com, Ok(Command::Expire("alex".to_string(), Duration::from_secs(3))));
    }
}
