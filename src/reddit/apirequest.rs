use futures::join;
use std::io;
use serde_json::Value;
use ureq;

pub async fn random_image(subreddit: String) -> String {
    let request_url = format!("https://reddit.com/r/{}/random.json", subreddit);
   
    let random = ureq::get(&request_url).call().into_string().unwrap();

    let request = parse_request(random).unwrap();

    return request;
}

pub async fn spam(subreddit: String) -> (String, String, String, String) {
    let request0 = random_image(subreddit.clone());
    let request1 = random_image(subreddit.clone());
    let request2 = random_image(subreddit.clone());
    let request3 = random_image(subreddit.clone());

    let results: (String, String, String, String) = join!(request0, request1, request2, request3);

    return results;
}

fn parse_request(api_data: String) -> io::Result<String> { 
    let v: Value = serde_json::from_str(&api_data)?;
            
    let url = String::from(format!("{}", v[0]["data"]["children"][0]["data"]["url"]));
              
    Ok(url)
}
