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
//! Code related to the B3 block.

use super::*;
use crate::print::{get_output_group, Interpretation};
use crate::ParsedUserInput;
use ratatui::layout::Rect;

pub fn draw_b3_block(f: &mut Frame, parent_rect: Rect, user_input: &ParsedUserInput) {
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

    draw_left(f, layout_b0[0], user_input);
    draw_right(f, layout_b0[2], user_input);
}

fn draw_left(f: &mut Frame, rect: Rect, user_input: &ParsedUserInput) {
    let output_group = get_output_group(user_input, Interpretation::SignedIntegers);

    let paragraph = output_group_to_widget(&output_group);
    f.render_widget(paragraph, rect)
}

fn draw_right(f: &mut Frame, rect: Rect, user_input: &ParsedUserInput) {
    let output_group = get_output_group(user_input, Interpretation::UnsignedIntegers);
    let paragraph = output_group_to_widget(&output_group);
    f.render_widget(paragraph, rect)
}
