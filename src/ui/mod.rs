pub mod events;
pub mod stats;
pub mod upgrades;

use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn create_layout(area: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // For event messages
            Constraint::Min(10),   // For the main content
        ])
        .split(area);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    (chunks[0], main_chunks[0])
}
