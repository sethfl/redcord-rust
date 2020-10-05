use std::fs::File;
use std::io;
use serde_json::Value;



pub fn random_image() -> io::Result<String> {
    let file = File::open("testing/reddit.json")
        .expect("failed to fetch api request!");
        
    println!("searching json file for media url...");
    
    let v: Value = serde_json::from_reader(file)?;
    
    let url = String::from(format!("{}", v[0]["data"]["children"][0]["data"]["url"]));
    
    if url.trim() == "Null" {
        let mut count = 0u32;
            
        loop {
            count += 1;
                
            println!("error finding a post with an image. trying again...");
                
            let file = File::open("/mnt/c/Users/setha/Desktop/dev/test2/src/reddit.json")?;
                
            let v: Value = serde_json::from_reader(file)?;
                
            let url = String::from(format!("{}", v[0]["data"]["children"][0]["data"]["url"]));
    
            if url.trim() != "Null" {
                break;
            }
                
            if count == 20 {
                panic!("couldn't find a post with an image. exiting...");
            }
        }
    }
    
    println!("media post found!");
    
    Ok(url)
}
