/// Application state for the chat TUI
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug)]
pub struct ChatApp {
    /// Current agent name
    pub agent: String,
    /// Message history
    pub messages: Vec<Message>,
    /// Current input buffer
    pub input: String,
    /// Scroll offset for message history
    pub scroll_offset: usize,
    /// Current mode (Chat, AgentSelect, Help, etc.)
    pub mode: AppMode,
    /// Connection status
    pub connected: bool,
    /// Whether the app should quit
    pub should_quit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Chat,
    AgentSelect,
    Help,
    Quit,
}

impl ChatApp {
    pub fn new() -> Self {
        Self {
            agent: "Nova".to_string(),
            messages: vec![
                Message {
                    sender: "System".to_string(),
                    content: "Welcome to Coven Chat! Type /help for commands, or just chat naturally."
                        .to_string(),
                    timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                },
            ],
            input: String::new(),
            scroll_offset: 0,
            mode: AppMode::Chat,
            connected: true,
            should_quit: false,
        }
    }

    pub fn add_message(&mut self, sender: impl Into<String>, content: impl Into<String>) {
        self.messages.push(Message {
            sender: sender.into(),
            content: content.into(),
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
        });
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.add_message("System", "Chat cleared. What's next?");
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset < self.messages.len().saturating_sub(1) {
            self.scroll_offset += 1;
        }
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
}
