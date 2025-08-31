use std::{time::{Duration, SystemTime}};

use kvdis::{command::Command, dictionary::{Dictionary, Entry}};

fn main() {
    //commmand::parse_line_to_command("SET key value").unwrap();

    let mut dict = Dictionary::new();

    dict.set(String::from("1"), Entry {
        value: String::from("bir"),
        expiration: None
    });
    dict.set(String::from("2"), Entry {
        value: String::from("iki"),
        expiration: None
    });
    dict.set(String::from("3"), Entry {
        value: String::from("üç"),
        expiration: Some(SystemTime::now() + Duration::from_secs(5))
    });

    dict.expire("3", Duration::from_secs(3));

    let set_command = "SET something whatevs".parse::<Command>().unwrap();
    let get_command = "GET something".parse::<Command>().unwrap();
    dict.run(set_command).unwrap();
    dict.run(get_command).unwrap();
    dbg!(dict.get("something").unwrap());
    dbg!(dict.exists("something"));


    //loop {
    //    let mut line = String::new();
    //    match io::stdin().read_line(&mut line) {
    //        Ok(_n_read) => {},
    //        Err(e) => {
    //            eprintln!("Could not read from stdin: {e}");
    //            continue;
    //        }
    //    };

    //}
}
