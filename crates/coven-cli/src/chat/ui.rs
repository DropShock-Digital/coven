/// UI rendering with Ratatui
use crate::chat::app::ChatApp;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub fn draw(f: &mut Frame, app: &ChatApp) {
    let size = f.area();

    // Main layout: [Header | Chat Area | Input Area]
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),   // Header
            Constraint::Min(5),       // Chat messages
            Constraint::Length(3),   // Input area
        ])
        .split(size);

    // Header
    draw_header(f, app, chunks[0]);

    // Chat area
    draw_chat_area(f, app, chunks[1]);

    // Input area
    draw_input_area(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &ChatApp, area: Rect) {
    let status = if app.connected { "●" } else { "○" };
    let header_text = format!(
        "   {}  Agent: {}  {}  Connection: {}",
        "🔮", app.agent, "|", status
    );

    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan).bold());

    f.render_widget(header, area);
}

fn draw_chat_area(f: &mut Frame, app: &ChatApp, area: Rect) {
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .rev()
        .skip(app.scroll_offset)
        .take(area.height as usize)
        .rev()
        .map(|msg| {
            let sender_color = if msg.sender == "You" {
                Color::Green
            } else if msg.sender == "System" {
                Color::Yellow
            } else {
                Color::Magenta
            };

            let line = format!("{}: {}", msg.sender, msg.content);
            ListItem::new(line).style(Style::default().fg(sender_color))
        })
        .collect();

    let message_list = List::new(messages)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White));

    f.render_widget(message_list, area);
}

fn draw_input_area(f: &mut Frame, app: &ChatApp, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(area);

    // Input hint
    let hint_text = if app.input.starts_with('/') {
        "Command (/ commands available)"
    } else {
        "Message or type / for commands"
    };

    let hint = Paragraph::new(hint_text)
        .style(Style::default().fg(Color::DarkGray).italic());
    f.render_widget(hint, chunks[0]);

    // Input field
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));

    let input_text = Paragraph::new(app.input.as_str())
        .block(input_block)
        .style(Style::default().fg(Color::White));

    f.render_widget(input_text, chunks[1]);

    // Show cursor
    if chunks[1].height > 0 {
        let cursor_x = chunks[1].x + 1 + (app.input.len() as u16).min(chunks[1].width.saturating_sub(2));
        let cursor_y = chunks[1].y + 1;
        f.set_cursor_position((cursor_x, cursor_y));
    }
}
