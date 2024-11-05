//! Code related to the B0 block.

use super::*;
use crate::print::{get_output_group, Interpretation};
use crate::ParsedUserInput;
use ratatui::layout::Rect;

pub fn draw_b1_block(f: &mut Frame<impl Backend>, parent_rect: Rect, user_input: &ParsedUserInput) {
    let layout_b0 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(2),
                Constraint::Percentage(38),
            ]
            .as_ref(),
        )
        .split(parent_rect);

    draw_left(f, layout_b0[0], user_input);
    draw_right(f, layout_b0[2], user_input);
}

fn draw_left(f: &mut Frame<impl Backend>, rect: Rect, user_input: &ParsedUserInput) {
    let output_group = get_output_group(user_input, Interpretation::NumeralSystems);

    let paragraph = output_group_to_widget(&output_group);
    f.render_widget(paragraph, rect)
}

fn draw_right(f: &mut Frame<impl Backend>, rect: Rect, user_input: &ParsedUserInput) {
    let output_group = get_output_group(user_input, Interpretation::IEEE754);
    let paragraph = output_group_to_widget(&output_group);
    f.render_widget(paragraph, rect)
}
