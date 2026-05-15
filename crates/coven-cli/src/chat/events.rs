/// Terminal event handling
use crate::chat::app::ChatApp;
use crate::chat::commands::{handle_command, parse_command};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub async fn handle_events(app: &mut ChatApp) -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            return Ok(handle_key_event(app, key));
        }
    }
    Ok(false)
}

fn handle_key_event(app: &mut ChatApp, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
            true
        }
        KeyCode::Char(c) => {
            app.input.push(c);
            false
        }
        KeyCode::Backspace => {
            app.input.pop();
            false
        }
        KeyCode::Enter => {
            let message = app.input.trim().to_string();
            if !message.is_empty() {
                if let Some(cmd) = parse_command(&message) {
                    handle_command(app, cmd);
                } else {
                    // Natural message
                    app.add_message("You", &message);
                    // TODO: Send to agent and get response
                    app.add_message(
                        &app.agent.clone(),
                        "[Response streaming would happen here]",
                    );
                }
                app.input.clear();
            }
            false
        }
        KeyCode::Up => {
            app.scroll_up();
            false
        }
        KeyCode::Down => {
            app.scroll_down();
            false
        }
        _ => false,
    }
}
