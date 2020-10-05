use directories::ProjectDirs;
use serde_json::Value;
use std::fs::File;
use std::io;
use std::mem;



pub fn print_out(word: String) {
    println!("{}", word);
}


pub fn cfg_file_check() -> io::Result<()> {
    if let Some(user) = ProjectDirs::from("com", "", "redcord") {
        let config_file = format!("{}/config.json", user.config_dir().display());

        let data = File::open(config_file)
            .expect("failed to read config file!)");

        let v: Value = serde_json::from_reader(data)?;

        let value = String::from(format!("{}", v["discord"]));

        print_out(value);
    }

    Ok(())
}

