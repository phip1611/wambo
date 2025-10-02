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

//! Everything related to the [`ratatui`] layout.

mod layout_b0;
mod layout_b1;
mod layout_b2;
mod layout_b3;
mod layout_b4;

use crate::ParsedUserInput;
use crate::print::OutputGroup;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{event, execute};
use layout_b0::*;
use layout_b1::*;
use layout_b2::*;
use layout_b3::*;
use layout_b4::*;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use std::io;

/// Displays the TUI and reacts to events, such as close.
/// Wrapper around [`draw_tui`].
pub fn run_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    user_input: &ParsedUserInput,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw_tui(f, user_input))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') ||
                // triggered when CTRL+C is pressed, as `crossterm` catches
                // all events and signals are not delivered normally
                key.code == KeyCode::Char('c')
            {
                return Ok(());
            }
        }
    }
}

/// Draw's the TUI.
pub fn draw_tui(f: &mut Frame, user_input: &ParsedUserInput) {
    // Constructs the main layout with 4 blocks:
    // - b0: heading block
    // - b1: numeral systems / f32/f64
    // - b2: kb / kib
    // - b3: signed integers / unsigned integers
    // - b4: bytes in memory
    let layout_main = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Length(6),
                Constraint::Length(7),
                Constraint::Length(7),
                Constraint::Length(4),
            ]
            .as_ref(),
        )
        .split(f.area());

    let border_block = Block::default().borders(Borders::NONE);

    f.render_widget(border_block.clone(), layout_main[0]);
    let inner_rect = border_block.clone().inner(layout_main[0]);
    draw_b0_block(f, inner_rect, user_input);

    f.render_widget(border_block.clone(), layout_main[1]);
    let inner_rect = border_block.clone().inner(layout_main[1]);
    draw_b1_block(f, inner_rect, user_input);

    f.render_widget(border_block.clone(), layout_main[2]);
    let inner_rect = border_block.clone().inner(layout_main[2]);
    draw_b2_block(f, inner_rect, user_input);

    f.render_widget(border_block.clone(), layout_main[3]);
    let inner_rect = border_block.clone().inner(layout_main[3]);
    draw_b3_block(f, inner_rect, user_input);

    f.render_widget(border_block.clone(), layout_main[3]);
    let inner_rect = border_block.clone().inner(layout_main[4]);
    draw_b4_block(f, inner_rect, user_input);
}

pub fn tui_prepare() -> io::Result<Terminal<impl Backend + io::Write>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn tui_cleanup(mut terminal: Terminal<impl Backend + io::Write>) -> io::Result<()> {
    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

/// Transforms the lines of an [`OutputGroup`] to a [`ratatui`]-compatible [`Paragraph`].
fn output_group_to_widget(output_group: &OutputGroup) -> Paragraph<'_> {
    let text = output_group
        .iter()
        // map each (key,value) pair to a Span
        .map(|(key, value)| {
            Line::from(vec![
                Span::styled(
                    format!("{}: ", key),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(value),
            ])
        })
        .collect::<Vec<_>>();

    Paragraph::new(text)
        .block(
            Block::default()
                .title(format!("{}", output_group.title()))
                .borders(Borders::NONE)
                .style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Blue),
                ),
        )
        .style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Black)
                // remove inherited bold from block styling
                .remove_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Left)
        // don't trim whitespaces, as this will break alignment
        .wrap(Wrap { trim: false })
}
