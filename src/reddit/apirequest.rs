use std::io;
use serde_json::Value;
use ureq;



pub fn random_image(subreddit: String) -> io::Result<String> {
    let request_url = format!("https://reddit.com/r/{}/random.json", subreddit);
   
    let random = ureq::get(&request_url).call().into_string().unwrap();

    let request = parse_request(random).unwrap();

    Ok(request)
}

pub fn parse_request(api_data: String) -> io::Result<String> { 
    let v: Value = serde_json::from_str(&api_data)?;
            
    let url = String::from(format!("{}", v[0]["data"]["children"][0]["data"]["url"]));
              
    Ok(url)
}
