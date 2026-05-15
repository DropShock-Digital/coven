/// Slash command parser and handler
use crate::chat::app::ChatApp;

#[derive(Debug, Clone)]
pub enum SlashCommand {
    Help,
    Clear,
    Agent(String),
    Exit,
    Unknown(String),
}

pub fn parse_command(input: &str) -> Option<SlashCommand> {
    if !input.starts_with('/') {
        return None;
    }

    let trimmed = input.trim_start_matches('/').trim();
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    match parts.get(0).copied() {
        Some("help") | Some("h") => Some(SlashCommand::Help),
        Some("clear") | Some("c") => Some(SlashCommand::Clear),
        Some("agent") | Some("a") => {
            let agent_name = parts.get(1).copied().unwrap_or("").to_string();
            if agent_name.is_empty() {
                Some(SlashCommand::Unknown("Usage: /agent <name>".to_string()))
            } else {
                Some(SlashCommand::Agent(agent_name.to_string()))
            }
        }
        Some("exit") | Some("q") | Some("quit") => Some(SlashCommand::Exit),
        Some(unknown) => Some(SlashCommand::Unknown(format!("Unknown command: /{}", unknown))),
        None => None,
    }
}

pub fn handle_command(app: &mut ChatApp, cmd: SlashCommand) {
    match cmd {
        SlashCommand::Help => {
            app.add_message(
                "System",
                "Available commands:\n\
                 /help - Show this help\n\
                 /clear - Clear chat history\n\
                 /agent <name> - Switch agent\n\
                 /exit - Quit\n\n\
                 Just type naturally to chat with the agent.",
            );
        }
        SlashCommand::Clear => {
            app.clear_messages();
        }
        SlashCommand::Agent(name) => {
            app.agent = name.clone();
            app.add_message("System", format!("Switched to agent: {}", name));
        }
        SlashCommand::Exit => {
            app.should_quit = true;
        }
        SlashCommand::Unknown(msg) => {
            app.add_message("System", msg);
        }
    }
}
