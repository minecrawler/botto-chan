use super::errors::*;


pub struct Command;

impl Command {
    pub fn interpret(user: String, command: String) -> Result<Option<String>> {
        match command {
            "!test" => Ok(Some(format!("@{} -> Test OK", user)),
            _ => Ok(None)
        }
    }
}
