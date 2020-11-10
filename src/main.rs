use discord::model::Event;
use discord::Discord;
use std::env;
use std::io;
mod config;
mod reddit;



fn main() -> io::Result<()> {
    config::directories::check_all();
    // config::setup::setup_init();
    
    let discord = Discord::from_bot_token(&env::var("DISCORD_TOKEN").expect("no discord token found as enviormental variable!"))
        .expect("bot login failed!");

    let (mut connection, _) = discord.connect().expect("failed to connect to discord!");
    println!("connected to discord!");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {

                if message.content.contains("-random") {
                    println!("recieved command from {}", message.author.name);
                    
                    let subreddit: String = message.content.replace("-random ", "");

                    println!("making a random_image API request from {}", subreddit);

                    let random_request = reddit::apirequest::random_image(subreddit).unwrap();

                    let url = format!("{}", random_request.trim_matches('"'));

                    println!("got {} from a random_image API request for {}", url, message.author.name); 

                    let _ = discord.send_message(
                        message.channel_id,
                        &url,
                        "",
                        false,
                        );
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("discord gateway closed with code {:?}: {}", code, body);
                break;
            }
        Err(err) => println!("failed to recieve data from discord! {:?}", err),
        }
    }
    Ok(())
}
