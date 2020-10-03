use directories::ProjectDirs;
use std::fs;
use std::path::Path;


pub fn config_check() {
    if let Some(user) = ProjectDirs::from("com", "", "redcord") {
        let config_exists = Path::new(user.config_dir()).is_dir();
        
        let config_path = user.config_dir();

        if config_exists == true {
        } else if config_exists == false {
            println!("configuration directory not found! creating directory...");
            match fs::create_dir_all(config_path) {
                Ok(()) => println!("created {}", config_path.display()),
                Err(error) => panic!("fucked up while making config_dir: {}", error),
            };
            println!("configuration directory has been created!")
        } else {
            println!("oh shit that's a big error in finding the configuration directory!!");
        }

        println!("configuration directory is set to {}", config_path.display());
    }
}


pub fn cache_check() {
    if let Some(user) = ProjectDirs::from("com", "", "redcord") {
        let cache_exists = Path::new(user.cache_dir()).is_dir();
        
        let cache_path = user.cache_dir();

        if cache_exists == true {
        } else if cache_exists == false {
            println!("cache directory not found! creating directory...");
            match fs::create_dir_all(cache_path) {
                Ok(()) => println!("created {}", cache_path.display()),
                Err(error) => panic!("eror while making cache_dir: {}", error),
            };
            println!("cache directory has been created!")
        } else {
            panic!("unknown error in finding the cache directory!!");
        }

        println!("cache directory is set to {}", cache_path.display());
    }
}


pub fn data_check() {
    if let Some(user) = ProjectDirs::from("com", "", "redcord") {
        let data_exists = Path::new(user.data_dir()).is_dir();
        
        let data_path = user.data_dir();

        if data_exists == true {
        } else if data_exists == false {
            println!("data directory not found! creating directory...");
            match fs::create_dir_all(data_path) {
                Ok(()) => println!("created {}", data_path.display()),
                Err(error) => panic!("fucked up while making data_dir: {}", error),
            };
            println!("data directory has been created!")
        } else {
            println!("oh shit that's a big error in finding the data directory!!");
        }

        println!("data directory is set to {}", data_path.display());
    }
}


pub fn check_all() {
    config_check();
    cache_check();
    data_check();
}
