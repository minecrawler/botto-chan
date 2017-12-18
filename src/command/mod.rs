use super::errors::*;


pub struct Command;

impl Command {
    pub fn interpret(user: String, command: String) -> Result<()> {
        println!("{} says: {}", message.author.name, message.content);
                if message.content == "!test" {
                    let _ = discord.send_message(message.channel_id, "This is a reply to the test.", "", false);
                } else if message.content == "!quit" {
                    println!("Quitting.");
                    break
                }
    }
}
