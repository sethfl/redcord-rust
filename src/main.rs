use discord::model::Event;
use discord::Discord;
use futures::executor::block_on;
use std::env;
use std::io;
mod reddit;

const PREFIX: char = '-';

fn main() -> io::Result<()> {
    let discord = Discord::from_bot_token(&env::var("DISCORD_TOKEN").expect("no discord token found as an environmental variable. please set it as DISCORD_TOKEN."))
        .expect("bot login failed!");

    let (mut connection, _) = discord.connect().expect("failed to connect to discord!");
    println!("connected to discord!");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                let random = format!("{}random", PREFIX);
                let spam = format!("{}spam", PREFIX);

                if message.content.contains(&random) {
                    let extra = format!("{} ", &random);
                    let subreddit: String = message.content.replace(&extra, "");

                    println!("got a random_image api request for {}", subreddit);

                    let random_request_future = reddit::apirequest::random_image(subreddit);

                    let random_request = block_on(random_request_future).unwrap();

                    let url = format!("{}", random_request.trim_matches('"'));

                    println!("got this for {}: {}", message.author.name, url); 

                    let _ = discord.send_message(
                        message.channel_id,
                        &url,
                        "",
                        false,
                        );
                } if message.content.contains(&spam) {
                    let extra = format!("{} ", &spam);
                    let subreddit: String = message.content.replace(&extra, "");
                    
                    println!("got a spam api request for {}", subreddit);

                    let mut i = 0;

                    while i < 10 {
                        let subreddit2: String = message.content.replace(&extra, "");

                        let random_request_future = reddit::apirequest::random_image(subreddit2);

                        let random_request = block_on(random_request_future).unwrap();

                        let url = format!("{}", random_request.trim_matches('"'));

                        println!("got this for {}: {}", message.author.name, url); 

                        let _ = discord.send_message(
                            message.channel_id,
                            &url,
                            "",
                            false,
                            );
                        i = i + 1;
                    }
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("discord gateway closed with code {:?}: {}", code, body);
                break;
            }
        Err(err) => println!("failed to receive data from discord! {:?}", err),
        }
    }
    Ok(())
}
