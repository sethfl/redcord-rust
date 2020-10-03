use directories::ProjectDirs;
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufWriter, stdin, Write};
use std::path::Path;



pub fn setup_config() {
    if let Some(user) = ProjectDirs::from("com", "", "redcord") {
        let config_file = format!("{}/{y}", user.config_dir().display(), y = "config.json");
        

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

        let buffer = File::open(config_file)
            .expect("unable to access cache!");
        let mut buffer = BufWriter::new(buffer);
        buffer.write_all(data.as_bytes())
            .expect("Unable to write to buffer!");

    }
}


pub fn check_values() { 
    if let Some(user) = ProjectDirs::from("com", "", "redcord") {
        let config_file = format!("{}/{y}", user.config_dir().display(), y = "config.json"); 
        
        if Path::new(&config_file).exists() == true {
            println!("configuration files exist. skipping setup...");
        } else {
            println!("no configuration files exist! entering setup...");
            setup_config();
        }
    }
}


pub fn setup_init() {
    check_values();
}
