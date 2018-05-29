//! Author: @truexpixels
//!
//! A simple Discord bot created using the Rust Serenity Framework.

#[macro_use] extern crate serenity;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {

    // Login via token from ENV file
    let mut client = Client::new(&env::var("DISCORD_TOKEN")
        .expect("token"), Handler) // Error Handling
        .expect("Error creating client"); // Error Handling
    client.with_framework(StandardFramework::new() // Implement Built-In Framework
        .configure(|c| c.prefix("rb!")) // set the bot's prefix to "~"
        .cmd("ping", ping) // Route to command macros
        .cmd("play", play)
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }

}

/// Sends a ping response back to the initial channel
command!(ping(_context, message) {
    let _ = message.channel_id.say(&format!("{}, Pong!", message.author.name));
});

/// Sets the bots status to the specified input
command!(play(_context, message) {
    use serenity::model::gateway::Game;
    use serenity::model::user::OnlineStatus;

    let content = message.content.replace("rb!play ", "");

    // Create Status Variables
    let game = Game::playing(&format!("{}", content));
    let status = OnlineStatus::DoNotDisturb;

    // Update Status
    _context.set_presence(Some(game), status);

    // Send Output
    let _ = message.channel_id.say(&format!("My game status has successfully updated to: **{}**", content));

});
