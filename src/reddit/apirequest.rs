use directories::ProjectDirs;
use std::fs;
use std::io;
use std::io::prelude::*;
use serde_json:: {from_str, Value};



pub fn save_request(request: str) {
    if let Some(user) = ProjectDirs::from("com", "", "redcord") {
        let save_location = format!("{}/{y}", user.cache_dir().display(), y = "requests.cache");

        let path = String::from(save_location);

        (fs:write(path, request)
            .expect("unable to cache request!"));
    }
}

pub fn random_image() -> io::Result<()> {
    let mut file = File::open("/mnt/c/Users/setha/Desktop/dev/test2/src/reddit.json")?;

    println!("searching json file for media url...");

    let v: Value = serde_json::from_reader(file)?;

    let mut url = println!("{}", v[0]["data"]["children"][0]["data"]["url"]);

    if String::from(url) == "Null" {
        let mut count == 0u32;
        
        loop {
            count += 1;
            
            println!("error finding a post with an image. trying again...");
            
            let file = File::open("/mnt/c/Users/setha/Desktop/dev/test2/src/reddit.json")?;
            
            let v: Value = serde_json::from_reader(file)?;
            
            let mut url = println!("{}", v[0]["data"]["children"][0]["data"]["url"])

            if String::from(url) != "Null" {
                break;
            }
            
            if count = 20 {
                panic!("couldn't find a post with an image. exiting...");
            }
        }
    }

    println!("media post found!");

    let request: str = serde_json::to_string(&url);

    save_request(request);

    Ok(())
}
