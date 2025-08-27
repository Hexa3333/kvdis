/*
    SET key value
    GET key
    DEL key
    EXISTS key
    EXPIRE key seconds
    INCR key
    DECR key
...
*/

use crate::errors::ParseError;


#[derive(Debug)]
enum CommandType {
    Set,
    Get,
    Del,
    Exists,
    Expire,
    Incr, Decr
}

/// Ensures line is a valid command
pub fn parse_line_to_command(line: &str) -> Result<(), ParseError> {
    let words: Vec<&str> = line.split_whitespace().collect();
    let c_type = word_to_command_type(&words[0])?;

    use CommandType::*;
    match c_type {
        Set => {
            if words.len() != 3 {
                dbg!("Found SET with lacking parameters");
                return Err(ParseError::InvalidParameters);
            } else {
                // Process SET
                dbg!("Found SET");
                todo!()
            }
        }

        _ => {
            todo!()
        }
    }
}

fn word_to_command_type(word: &str) -> Result<CommandType, ParseError> {
    use CommandType::*;
    match word {
        "SET" =>    Ok(Set),
        "GET" =>    Ok(Get),
        "DEL" =>    Ok(Del),
        "EXISTS" => Ok(Exists),
        "EXPIRE" => Ok(Expire),
        "INCR" =>   Ok(Incr),
        "DECR" =>   Ok(Decr),

        _ => Err(ParseError::NotACommand)
    }
}
