use std::io;
mod config;
mod reddit;
//use serenity::async_trait;
//use serenity::client::{Client, Context, EventHandler};
//use serenity::model::channel::Message;
//use serenity::framework::standard::{
//    StandardFramework,
//    CommandResult,
//    macros::{
//        command,
//        group
//    }
//};

fn main() {
    // config::directories::check_all();
    // config::setup::setup_init();
    println!("what is the subreddit you want a post from? (without the /r/)");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin!");
    let url = reddit::apirequest::random_image(input).unwrap();
    println!("{}", url.trim_matches('"'));
}
