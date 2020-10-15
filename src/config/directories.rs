use dirs;
use std::fs;
use std::path::Path;



pub fn config_check() {
    let detected_config = format!("{:?}", dirs::config_dir().unwrap());

    let config_path = format!("{}/redcord", detected_config.trim_matches('"'));

    let config_print = format!("{}/redcord", detected_config.trim_matches('"'));
    
    let config_exists = Path::new(&config_path).is_dir();

    if config_exists == true {
        println!("configuration directory is set to {}", config_path);
    } else {
        println!("configuration directory not found! creating directory...");
        fs::create_dir_all(config_path)
            .expect("unable to create config directory!");
        println!("configuration directory has been created!");
        println!("configuration directory is set to {}", config_print);
    }
}


pub fn cache_check() {
    let detected_cache = format!("{:?}", dirs::cache_dir().unwrap());

    let cache_path = format!("{}/redcord", detected_cache.trim_matches('"'));
    
    let cache_print = format!("{}/redcord", detected_cache.trim_matches('"'));

    let cache_exists = Path::new(&cache_path).is_dir();

    if cache_exists == true {
        println!("cache directory is set to {}", cache_path);
    } else {
        println!("cache directory not found! creating directory...");
        fs::create_dir_all(cache_path)
            .expect("unable to create cache directory!");
        println!("cache directory has been created!");
        println!("cache directory is set to {}", cache_print);
    }
}


pub fn data_check() {
    let detected_data = format!("{:?}", dirs::data_dir().unwrap());

    let data_path = format!("{}/redcord", detected_data.trim_matches('"'));

    let data_print = format!("{}/redcord", detected_data.trim_matches('"'));
    
    let data_exists = Path::new(&data_path).is_dir();

    if data_exists == true {
        println!("data directory is set to {}", data_path);
    } else {
        println!("data directory not found! creating directory...");
        fs::create_dir_all(data_path)
            .expect("unable to create data path!");
        println!("data directory has been created!");
        println!("data directory is set to {}", data_print);
    }
}


pub fn check_all() {
    config_check();
    cache_check();
    data_check();
}
