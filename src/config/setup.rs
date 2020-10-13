use dirs;
use serde_json::{json, Value};
use std::fs;
use std::io::{Error, self, stdin, Write};
use std::path::Path;


pub fn setup_config() {
    let config = dirs::config_dir();

    let config_file = format!("{:?}/config.json", config.unwrap()); 
    
    let mut reddit_id = String::new();
    let mut reddit_secret = String::new();
    let mut discord = String::new();
         
    println!("please enter your reddit application client_id: ");
    stdin()
        .read_line(&mut reddit_id)
        .expect("failed to read reddit_id!");

    println!("now enter your reddit application secret key: ");
    stdin()
        .read_line(&mut reddit_secret)
        .expect("failed to read reddit_secret!");

    println!("finally, enter your discord bot secret key: ");
    stdin()
        .read_line(&mut discord)
        .expect("failed to read discord_secret!");
    
    let data_pre: Value = json!({
        "reddit_id": reddit_id,
        "reddit_secret": reddit_secret,
        "discord": discord
    });
        
    let data = format!("{}", data_pre.to_string());

    let mut config_save = fs::File::create(config_file)
        .expect("unable to create config file!");

    write!(config_save, "{}", data)
        .expect("unable to write config file!");
}


pub fn check_values() { 
    let config = dirs::config_dir();

    let config_file = format!("{:?}/config.json", config.unwrap()); 
    
    if Path::new(&config_file).exists() == true {
        println!("configuration files exist. skipping setup...");
    } else {
        println!("no configuration files exist! entering setup...");
        setup_config();
    }
}


pub fn setup_init() {
    check_values();
}
