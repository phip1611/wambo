/*
MIT License

Copyright (c) 2020 Philipp Schuster

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

//! Everything related to the [`tui`] layout.

mod layout_b0;
mod layout_b1;
mod layout_b2;
mod layout_b3;
mod layout_b4;

use crate::print::OutputGroup;
use crate::ParsedUserInput;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use layout_b0::*;
use layout_b1::*;
use layout_b2::*;
use layout_b3::*;
use layout_b4::*;
use std::io;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::{Frame, Terminal};

/// Displays the TUI and reacts to events, such as close.
/// Wrapper around [`draw_tui`].
pub fn run_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    user_input: &ParsedUserInput,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw_tui(f, user_input))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
            // triggered when CTRL+C is pressed, as `crossterm` catches
            // all events and signals are not delivered normally
            else if let KeyCode::Char('c') = key.code {
                return Ok(());
            }
        }
    }
}

/// Draw's the TUI.
pub fn draw_tui(f: &mut Frame<impl Backend>, user_input: &ParsedUserInput) {
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
                Constraint::Length(3),
                Constraint::Length(6),
                Constraint::Length(7),
                Constraint::Length(7),
                Constraint::Length(4),
            ]
            .as_ref(),
        )
        .split(f.size());

    let border_block = Block::default().borders(Borders::NONE);

    f.render_widget(border_block.clone(), layout_main[0].clone());
    let inner_rect = border_block.clone().inner(layout_main[0].clone());
    draw_b0_block(f, inner_rect, user_input);

    f.render_widget(border_block.clone(), layout_main[1].clone());
    let inner_rect = border_block.clone().inner(layout_main[1].clone());
    draw_b1_block(f, inner_rect, user_input);

    f.render_widget(border_block.clone(), layout_main[2].clone());
    let inner_rect = border_block.clone().inner(layout_main[2].clone());
    draw_b2_block(f, inner_rect, user_input);

    f.render_widget(border_block.clone(), layout_main[3].clone());
    let inner_rect = border_block.clone().inner(layout_main[3].clone());
    draw_b3_block(f, inner_rect, user_input);

    f.render_widget(border_block.clone(), layout_main[3].clone());
    let inner_rect = border_block.clone().inner(layout_main[4].clone());
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

/// Transforms the lines of an [`OutputGroup`] to a [`tui`]-compatible [`Paragraph`].
fn output_group_to_widget(output_group: &OutputGroup) -> Paragraph {
    let text = output_group
        .iter()
        .map(|(key, value)| {
            Spans::from(vec![
                Span::styled(
                    format!("{key}: "),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!("{value}")),
            ])
        })
        .collect::<Vec<_>>();

    let paragraph = Paragraph::new(text)
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
        .wrap(Wrap { trim: false });

    paragraph
}
