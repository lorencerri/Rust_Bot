//! Author: @truexpixels
//!
//! A simple Discord bot created using the Rust Serenity Framework.

#[macro_use]
extern crate serenity;
extern crate chrono;

use chrono::prelude::*;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Game;
use serenity::model::user::OnlineStatus;
use serenity::prelude::EventHandler;
use serenity::utils::Colour;
use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {

    // Login via token from ENV file
    let mut client = Client::new(&env::var("DISCORD_TOKEN")
        .expect("token"), Handler) // Error Handling
        .expect("Error creating client"); // Error Handling
    client.with_framework(
        StandardFramework::new() // Implement Built-In Framework
        .configure(|c| c.prefix("rb!").case_insensitivity(true).ignore_bots(true)) // set the bot's prefix to "~"
        .cmd("ping", ping) // Route to command macros
        .cmd("play", play)
        .cmd("info", info)
        .cmd("commands", commands)
        .cmd("time", time)
        .cmd("date", time)
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(time(_context, message) {

    let mut output_text = "time";
    let mut timezone = "UTC";

    if message.content.to_string().to_uppercase().find("DATE") != None {
        output_text = "date";
    }

    // Output Final Message
    let _ = message.channel_id.send_message(|m| m
        .embed(|e| e
            .color(Colour::blurple())
            .title(&format!("The current {} in {} is {}", output_text, timezone, Utc::now().to_rfc2822().replace("+0000", "")))
    ));

});

// Returns some information about the commands
command!(commands(_context, message) {

    let prefix = "rb!";

    // General Commands
    let mut general = format!("**`{}commands`** - *Displays all available commands*\n", prefix).to_string();
    general.push_str(&format!("**`{}info`** - *Displays basic information relating to the bot*\n", prefix));
    general.push_str(&format!("**`{}ping`** - *Returns pong*\n", prefix));
    general.push_str(&format!("**`{}play game`** - *Sets the bots presence as the input*", prefix));
    general.push_str(&format!("**`{}time`** - *Displays the current time in UTC*", prefix));

    // Output Final Message
    let _ = message.channel_id.send_message(|m| m
        .embed(|e| e
            .title("Commands")
            .color(Colour::blurple())
            .description(&format!("The default prefix for this bot is **`{}`**, and the commands are case-insensitive.", prefix))
            .field("General Commands", general, true)
    ));

});

/// Returns some information about to the bot to the initial channel
command!(info(_context, message) {

    // Variables
    let github = "https://github.com/TrueXPixels/Rust_Bot";
    let name = "Rust Bot";

    // Create & Send Embed
    let _ = message.channel_id.send_message(|m| m
        .embed(|e| e
            .title("Information")
            .color(Colour::blurple())
            .description(&format!("**{}** is an open-sourced bot created using the **Rust** programming language.", name))
            .field("Repository", github, false)
    ));

});

/// Sends a ping response back to the initial channel
command!(ping(_context, message) {

    // Create & Sent Embed
    let _ = message.channel_id.send_message(|m| m
        .embed(|e| e
            .title(&format!("{}, Pong!", message.author.name))
            .color(Colour::blurple())
    ));

});

/// Sets the bots status to the specified input
command!(play(_context, message) {

    // Defines content, and replaces the rb!play in the text with nothing
    let content = message.content.replace("rb!play ", "");

    // Create Status Variables
    let game = Game::playing(&format!("{}", content));
    let status = OnlineStatus::DoNotDisturb;

    // Update Status
    _context.set_presence(Some(game), status);

    // Send Output
    let _ = message.channel_id.say(&format!("My game status has successfully updated to: **{}**", content));

});
