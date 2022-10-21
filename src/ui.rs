use tui::Frame;
use tui::layout::{Alignment, Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::backend::{Backend};
use tui::widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap};
use crate::{App, lib};

pub fn build_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(1)
        ].as_ref())
        .split(size);

    let paragraph = Paragraph::new(Span::styled(
        app.image_name.clone(),
        Style::default().add_modifier(Modifier::BOLD),
    ))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);

    let image_block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Image",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(app.image_name.to_string()).block(image_block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);

    let selected_style = Style::default().bg(Color::Green).fg(Color::DarkGray).add_modifier(Modifier::ITALIC);
    let mut rows = vec![];
    for i in &app.trivy.results {
        match &i.vulnerabilities {
            Some(vul) => {
                rows.push((Row::new(vec![
                    Span::styled("", Style::default()),
                ]), std::ptr::null()));
                rows.push((Row::new(vec![
                    Span::styled("", Style::default()),
                    Span::styled("target: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    Span::styled(i.target.clone(), Style::default().fg(Color::Blue)),
                ]), std::ptr::null()));
                rows.push((Row::new(vec![
                    Span::styled("", Default::default()),
                ]), std::ptr::null()));
                for j in vul {
                    rows.push((Row::new(vec![
                        Cell::from(j.severity.clone().unwrap_or("None".to_string())).style(lib::critical_color(j.severity.clone().unwrap_or("None".to_string()))),
                        Cell::from(j.vulnerability_id.clone().unwrap_or("None".to_string())).style(Style::default().fg(Color::White)),
                        Cell::from(j.title.clone().unwrap_or("None".to_string())).style(Style::default().fg(Color::White)),
                    ]), j));
                }
            }
            _ => {}
        }
    }
    let (v1, _): (Vec<_>, Vec<_>) = rows.clone().into_iter().unzip();
    let t = Table::new(v1.clone())
        .column_spacing(1)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .style(lib::check_focus(app.show_popup))
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(60),
        ]);
    f.render_stateful_widget(t, chunks[1], &mut app.state);

    let footer = Paragraph::new("Trivy UI: Use arrow keys to navigate and press ESC to exit screens!").style(Style::default().fg(Color::Yellow)).alignment(Alignment::Left);
    f.render_widget(footer, chunks[2]);

    if app.show_popup {
        let selected_row = app.state.selected().unwrap_or(0);
        unsafe {
            let row = rows.get(selected_row).expect("No row found");
            let vul = row.1;
            if !vul.is_null() {
                let block = Block::default().title(format!("Summary for {}", (*vul).vulnerability_id.as_ref().expect("dw"))).border_style(Style::default().fg(Color::Green)).borders(Borders::ALL);

                let mut text = vec![
                    Spans::from(""),
                    Spans::from(vec![
                        Span::styled("Title:", Style::default().fg(Color::Green)),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::raw((*vul).title.as_ref().expect("error getting title")),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::styled("Description:", Style::default().fg(Color::Green)),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::raw((*vul).description.as_ref().expect("error getting title")),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::styled("Vulnerability ID:", Style::default().fg(Color::Green)),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::raw((*vul).vulnerability_id.as_ref().expect("error getting title")),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::styled("Severity:", Style::default().fg(Color::Green)),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::raw((*vul).severity.as_ref().expect("error getting title")),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::styled("Severity Source:", Style::default().fg(Color::Green)),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::raw((*vul).severity_source.as_ref().expect("error getting title")),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::styled("Package Name:", Style::default().fg(Color::Green)),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::raw((*vul).pkg_name.as_ref().expect("error getting title")),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::styled("Installed Version:", Style::default().fg(Color::Green)),
                    ]),
                    Spans::from(""),
                    Spans::from(vec![
                        Span::raw((*vul).installed_version.as_ref().expect("error getting title")),
                    ]),
                    Spans::from(""),
                ];

                if let Some(ref fixed_version) = (*vul).fixed_version {
                    text.push(
                        Spans::from(vec![
                            Span::styled("Fixed Version:", Style::default().fg(Color::Green)),
                        ]));
                    text.push(Spans::from(""));
                    text.push(Spans::from(vec![
                        Span::raw(fixed_version),
                    ]));
                }
                let paragraph = Paragraph::new(text).block(block.clone()).wrap(Wrap { trim: false }).scroll((app.pop_scroll, 0));
                let area = lib::centered_rect(60, 40, size);
                f.render_widget(Clear, area);
                f.render_widget(paragraph, area);
                f.render_widget(block, area);
            } else {
                app.show_popup = false;
                build_ui(f, app);
            }
        }
    }
}
