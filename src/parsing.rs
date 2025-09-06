use std::str::FromStr;
use crate::errors::ParseError;
use crate::command::Command;

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.len() == 0 {
            return Err(ParseError::IsEmpty);
        }

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
            },
            "EXISTS" => {
                if words.len() != 2 {
                    return Err(ParseError::InvalidParameters);
                } else {
                    Ok(Exists(words[1].to_string()))
                }
            }
            "EXPIRE" => {
                // NOTE: humantime format is standard.
                if words.len() < 3 || words.len() > 5 {
                    return Err(ParseError::InvalidParameters);
                } else {
                    let humantime_part = match words.get(2..) {
                        Some(humantime_part) => humantime_part,
                        None => {
                            return Err(ParseError::InvalidParameters);
                        }
                    };
                    let humantime_string = humantime_part.join(" ");

                    Ok(Expire(words[1].to_string(), humantime_string.parse::<humantime::Duration>().map_err(|_e| {
                        ParseError::InvalidParameters
                    })?.into()))
                }
            }
            "INCR" => {
                if words.len() != 2 {
                    return Err(ParseError::InvalidParameters);
                } else {
                    Ok(Incr(words[1].to_string()))
                }
            }
            "DECR" => {
                if words.len() != 2 {
                    return Err(ParseError::InvalidParameters);
                } else {
                    Ok(Decr(words[1].to_string()))
                }
            },
            "CLEAR" => {
                Ok(Clear)
            },
            "SAVE" => {
                Ok(Save)
            },
            "LOAD" => {
                Ok(Load)
            }

            _ => Err(ParseError::NotACommand)
        }
    }
}

#[cfg(test)]
mod parsing {
    use super::*;
    use std::time::Duration;

    #[test]
    fn expire_single_var() {
        let s = "EXPIRE alex 3s";
        let com = s.parse::<Command>();

        assert_eq!(com, Ok(Command::Expire("alex".to_string(), Duration::from_secs(3))));
    }

    #[test]
    fn set() {
        let key = "metanoia";
        let value = "19";

        let com = String::from("SET ") + key + " " + value;
        let com = com.parse::<Command>();

        assert_eq!(com, Ok(Command::Set("metanoia".to_string(), "19".to_string())));
    }

    #[test]
    fn get() {
        let key = "metanoia";

        let com = String::from("GET ") + key;
        let com = com.parse::<Command>();

        assert_eq!(com, Ok(Command::Get("metanoia".to_string())));
    }

    #[test]
    fn del() {
        let key = "metanoia";

        let com = String::from("DEL ") + key;
        let com = com.parse::<Command>();

        assert_eq!(com, Ok(Command::Del("metanoia".to_string())));
    }

    #[test]
    fn exists() {
        let key = "metanoia";

        let com = String::from("EXISTS ") + key;
        let com = com.parse::<Command>();

        assert_eq!(com, Ok(Command::Exists("metanoia".to_string())));
    }

    #[test]
    fn expire_multiple_var() {
        let key = "metanoia";

        let com = String::from("EXPIRE ") + key + " 1h 27m 13s";
        let com = com.parse::<Command>();

        // 1h 27m 13s is 5223 seconds
        assert_eq!(com, Ok(Command::Expire("metanoia".to_string(), std::time::Duration::from_secs(5_233))));
    }

    #[test]
    fn incr() {
        let key = "metanoia";

        let com = String::from("INCR ") + key;
        let com = com.parse::<Command>();

        assert_eq!(com, Ok(Command::Incr("metanoia".to_string())));
    }

    #[test]
    fn decr() {
        let key = "metanoia";

        let com = String::from("DECR ") + key;
        let com = com.parse::<Command>();

        assert_eq!(com, Ok(Command::Decr("metanoia".to_string())));
    }
}
