//! Code related to the B0 block.

use super::*;
use crate::ParsedUserInput;
use ratatui::layout::Rect;
use ratatui::text::Line;

pub fn draw_b0_block(f: &mut Frame<impl Backend>, parent_rect: Rect, user_input: &ParsedUserInput) {
    let layout_b0 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(49),
                Constraint::Percentage(2),
                Constraint::Percentage(49),
            ]
            .as_ref(),
        )
        .split(parent_rect);

    draw_left(f, layout_b0[0]);
    draw_right(f, layout_b0[2], user_input);
}

fn draw_left(f: &mut Frame<impl Backend>, rect: Rect) {
    let text = vec![
        Line::from(vec![Span::styled(
            format!("WAMBO (v{})", env!("CARGO_PKG_VERSION")),
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::raw("Web version: "),
            Span::styled("https://wambo-web.de", Style::default().fg(Color::Yellow)),
        ]),
    ];
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("").borders(Borders::NONE))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, rect)
}

fn draw_right(f: &mut Frame<impl Backend>, rect: Rect, user_input: &ParsedUserInput) {
    let text = vec![Line::from(vec![
        Span::styled("Input: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(user_input.normalized_input()),
    ])];
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("").borders(Borders::NONE))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, rect)
}
