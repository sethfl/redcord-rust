use dirs;
use std::fs;
use std::path::Path;



pub fn config_check() {
    let config_path = format!("{:?}", dirs::config_dir().unwrap());

    let config_exists = Path::new(&dirs::config_dir().unwrap()).is_dir();

    if config_exists == true {
        println!("configuration directory is set to {}", config_path);
    } else {
        println!("configuration directory not found! creating directory...");
        fs::create_dir_all(config_path);
        println!("configuration directory has been created!");
        println!("configuration directory is set to {:?}", dirs::config_dir().unwrap());
    }
}


pub fn cache_check() {
    let cache_path = format!("{:?}", dirs::cache_dir().unwrap());

    let cache_exists = Path::new(&dirs::cache_dir().unwrap()).is_dir();

    if cache_exists == true {
        println!("cache directory is set to {}", cache_path);
    } else {
        println!("cache directory not found! creating directory...");
        fs::create_dir_all(cache_path);
        println!("cache directory has been created!");
        println!("cache directory is set to {:?}", dirs::cache_dir().unwrap());
    }
}


pub fn data_check() {
    let data_path = format!("{:?}", dirs::data_dir().unwrap());

    let data_exists = Path::new(&dirs::data_dir().unwrap()).is_dir();

    if data_exists == true {
        println!("data directory is set to {}", data_path);
    } else {
        println!("data directory not found! creating directory...");
        fs::create_dir_all(data_path);
        println!("data directory has been created!");
        println!("data directory is set to {:?}", dirs::data_dir().unwrap());
    }
}


pub fn check_all() {
    config_check();
    cache_check();
    data_check();
}
