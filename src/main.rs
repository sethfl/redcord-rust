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

                    let random_request = block_on(random_request_future);

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

                    let ready = String::from("get ready...");
                    
                    let _ = discord.send_message(
                        message.channel_id,
                        &ready,
                        "",
                        false,
                    );

                    println!("loading...");

                    let spam = block_on(reddit::apirequest::spam(subreddit));

                    println!("done! sending urls...");

                    let url0 = format!("{:?}", spam.0.trim_matches('"'));
                    let url1 = format!("{:?}", spam.1.trim_matches('"'));
                    let url2 = format!("{:?}", spam.2.trim_matches('"'));
                    let url3 = format!("{:?}", spam.3.trim_matches('"'));

                    let _ = discord.send_message(
                        message.channel_id,
                        &url0.trim_matches('"'),
                        "",
                        false,
                    );
                    let _ = discord.send_message(
                        message.channel_id,
                        &url1.trim_matches('"'),
                        "",
                        false,
                    );
                    let _ = discord.send_message(
                        message.channel_id,
                        &url2.trim_matches('"'),
                        "",
                        false,
                    );
                    let _ = discord.send_message(
                        message.channel_id,
                        &url3.trim_matches('"'),
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
        Err(err) => println!("failed to receive data from discord! {:?}", err),
        }
    }
    Ok(())
}
