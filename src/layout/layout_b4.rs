//! Code related to the B0 block.

use super::*;
use crate::print::{get_output_group, Interpretation};
use crate::ParsedUserInput;
use ratatui::layout::Rect;

pub fn draw_b4_block(f: &mut Frame, parent_rect: Rect, user_input: &ParsedUserInput) {
    let output_group = get_output_group(user_input, Interpretation::Bit64BigEndian);

    let paragraph = output_group_to_widget(&output_group);
    f.render_widget(paragraph, parent_rect)
}
