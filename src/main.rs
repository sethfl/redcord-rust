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
    config::directories::check_all();
    config::setup::setup_init();
    let url = reddit::apirequest::random_image().unwrap();
    println!("{}", url.trim_matches('"'));
}
