use dotenv::dotenv;
use std::env;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::borrow::Borrow;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        if new_message.content == "!hw" {
            let channel = match new_message.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {why:?}");
                    return;
                }
            };
            let response = MessageBuilder::new()
                .push_line("haiii haiii XD haiii!!!! :3c")
                .push_line("nyoo~ hello, fwends~! :3")
                .push_line("# Hawk Tuah.")
                .push("waoh .... nice server  you got here .... x3").build();

            if let Err(why) = new_message.channel_id.say(&ctx.http, &response).await {
                println!("Error sending message: {why:?}");
            }
        }
        else if new_message.content == "!nightly" {
            let channel = match new_message.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {why:?}");
                    return;
                }
            };
            let filename = Path::new("nightly.txt");
            if let Err(why) = read_lines(filename) {
                println!("Error reading file: {filename:?} {why:?}");
            }

            if let Ok(lines) = read_lines(filename) {
                let mut nightly = MessageBuilder::new();
                for line in lines.map_while(Result::ok) {
                    nightly.push_line(line);
                }
                nightly.build();
                let asd = match new_message.channel_id.say(&ctx.http, &nightly.to_string()).await {
                    Ok(g) => { return; }
                    Err(why) => {
                        println!("Error getting channel: {why:?}");
                        return;
                    }
                };
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    println!("Hello, world!");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // match client.start().await {
    //     Err(why) => println!("Bot failed to start: {why:?}"),
    //    Ok(lol) => {println!("Bot started properly"); lol }
    // };
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    } else {
        println!("Bot started properly");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}