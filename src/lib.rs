use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};

pub fn critical_color(crtical: String) -> Style {
    return if crtical == "CRITICAL" {
        Style::default().fg(Color::Red)
    } else if crtical == "HIGH" {
        Style::default().fg(Color::Yellow)
    } else if crtical == "MEDIUM" {
        Style::default().fg(Color::Blue)
    } else if crtical == "LOW" {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::White)
    };
}

pub fn check_focus(popup: bool) -> Style {
    return if popup {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::Green)
    };
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
                .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
                .as_ref(),
        )
        .split(popup_layout[1])[1]
}
