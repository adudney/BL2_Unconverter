extern crate regex;

use std::env;
use std::fs::File;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut file = None;
    for arg in env::args() {
        let arg = arg.split('=').collect::<Vec<&str>>();
        match arg[0] {
            "-f" => { // File
                file = Some(File::open(arg[1]).unwrap());
            },
            _ => {
            }
        }
    }
    if let None = file {
        return;
    }
    let mut file = file.unwrap();
    let mut val = String::new();
    file.read_to_string(&mut val).unwrap();

    let keys_regex = Regex::new(r"set Transient.SparkServiceConfiguration_6 Keys \((.*?)\)").unwrap();
    let keys_string = keys_regex.captures(&val).unwrap();

    let key_regex = Regex::new(r#""Spark(.*?)PatchEntry(.*?)""#).unwrap();
    let mut keys = Vec::new();

    for key in key_regex.captures_iter(&keys_string[1]) {
        keys.push((key[1].to_string(), key[2].to_string()));
    }

    let values_regex = Regex::new(r"set Transient.SparkServiceConfiguration_6 Values \((.*?)\)\s").unwrap();
    let values_string = values_regex.captures(&val).unwrap();

    let value_regex = Regex::new(r#""(.*?)""#).unwrap();
    let mut values = Vec::new();

    for value in value_regex.captures_iter(&values_string[1]) {
        values.push(value[1].to_string());
    }

    if values.len() != keys.len() {
        println!("# WARNING: Values length ({}) != keys length ({})", values.len(), keys.len());
    }

    let mut current_type = "".to_string();
    let mut current_class = "".to_string();
    let mut switched = false;

    for (&(ref key_type, ref key_name), value) in keys.iter().zip(values.iter()) {
        println!("# {}", key_name);
        let mut value_split = value.split(',');
        if &current_type != key_type {
            current_type = key_type.to_string();
            switched = true;
        }

        match current_type.to_lowercase().as_str() {
            "ondemand" => {
                let val = value_split.next().unwrap();
                if val != current_class {
                    current_class = val.to_string();
                    switched = true;
                }
            },
            "level" => {
                let val = value_split.next().unwrap();
                if val != current_class {
                    current_class = val.to_string();
                    switched = true;
                }
            },
            "" => {
                current_class = "".to_string();
            },
            _ => {
                println!("# Warning, unrecognized key type");
            }
        }

        if switched {
            print!("start {}", current_type);
            match current_type.to_lowercase().as_str() {
                "ondemand" => {
                    println!(" {}", current_class);
                },
                "level" => {
                    if current_class == "" {
                        println!(" None");
                    } else {
                        println!(" {}", current_class);
                    }
                },
                "" => {
                    println!("Patch");
                },
                _ => {
                    println!("");
                }
            }
            switched = false;
        }

        let value_split = value_split.collect::<Vec<&str>>();
        if value_split[2] != "" {
            print!("set_cmp");
        } else {
            print!("set");
        }

        for split in value_split {
            if split == "" {
                continue;
            }
            print!(" {}", split);
        }
        println!("");
    }
}
