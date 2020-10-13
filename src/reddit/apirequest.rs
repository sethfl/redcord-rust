use dirs;
use std::fs;
use std::io::{Read, self, Write};
use serde_json::Value;
use ureq;



pub fn random_image(subreddit: String) -> io::Result<String> {
    let request_url = format!("https://reddit.com/r/{}/random.json", subreddit);
   
    let random_apirequest = ureq::get(&request_url).call().into_string().unwrap();

    cache_request(random_apirequest);

    let request = parse_request().unwrap();

    Ok(request)
}


pub fn cache_request(request: String) {
    let cache = dirs::cache_dir();

    let target = format!("{:?}/random.json", cache.unwrap());

    println!("caching API request...");

    fs::write(target, request)
            .expect("unable to cache API request!");
}


pub fn parse_request() -> io::Result<String> { 
    let cache = dirs::cache_dir();

    let target = format!("{:?}/random.json", cache.unwrap());

    println!("searching json file for media url...");
    
    let file = fs::File::open(target)
        .expect("unable to read API request!");
    
    let v: Value = serde_json::from_reader(file)?;
            
    let url = String::from(format!("{}", v[0]["data"]["children"][0]["data"]["url"]));
              
    Ok(url)
}
