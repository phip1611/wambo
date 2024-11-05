/*
MIT License

Copyright (c) 2024 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
//! Code related to the B0 block.

use super::*;
use crate::ParsedUserInput;
use ratatui::layout::Rect;
use ratatui::text::Line;

pub fn draw_b0_block(f: &mut Frame, parent_rect: Rect, user_input: &ParsedUserInput) {
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

fn draw_left(f: &mut Frame, rect: Rect) {
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

fn draw_right(f: &mut Frame, rect: Rect, user_input: &ParsedUserInput) {
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
