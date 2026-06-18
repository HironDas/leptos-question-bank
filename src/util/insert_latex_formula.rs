use leptos::html;
use leptos::prelude::*;

pub enum MathSyntex {
    Times,
    Division,
    Neq,
    Approx,
    Pm,
    Frac,
    Sub,
    Sup,
    Root,
    Pi,
    NewLine,
}

pub fn insert_formula(textarea_ref: NodeRef<html::Textarea>, syntex: MathSyntex) {
    if let Some(textarea) = textarea_ref.get_untracked() {
        let value = textarea.value();

        // 1. Fetch JS UTF-16 character position metrics
        let char_start = textarea.selection_start().unwrap_or(None).unwrap_or(0) as usize;
        let char_end = textarea.selection_end().unwrap_or(None).unwrap_or(0) as usize;

        // 2. Safely translate character positions to safe Rust byte offsets
        let mut byte_start = value.len();
        let mut byte_end = value.len();

        for (char_idx, (byte_idx, _)) in value.char_indices().enumerate() {
            if char_idx == char_start {
                byte_start = byte_idx;
            }
            if char_idx == char_end {
                byte_end = byte_idx;
                break; // Boundary constraints fulfilled, stop loop execution
            }
        }

        let replacement = match syntex {
            MathSyntex::Times => format!(" \\times "),
            MathSyntex::Division => format!(" \\div "),
            MathSyntex::Frac => format!(" \\frac{{}}{{}}"),
            MathSyntex::Approx => format!(" \\approx "),
            MathSyntex::Neq => format!(" \\neq "),
            MathSyntex::NewLine => format!("\\\\ \n"),
            MathSyntex::Pi => format!("\\pi"),
            MathSyntex::Pm => format!("\\pm "),
            MathSyntex::Root => format!("\\sqrt[]{{}}"),
            MathSyntex::Sub => format!("_"),
            MathSyntex::Sup => format!("^"),
        };

        // 3. Assemble the final string text using the calculated byte layout splits
        let new_value = format!(
            "{}{}{}",
            &value[0..byte_start],
            replacement,
            &value[byte_end..]
        );
        textarea.set_value(&new_value);

        let _ = textarea.focus();

        // 4. Calculate new cursor coordinates using character lengths (since JS needs characters)
        let replacement_char_len = replacement.chars().count();
        let new_cursor_pos = (char_start + replacement_char_len) as u32;

        let _ = textarea.set_selection_range(new_cursor_pos, new_cursor_pos);
    }
}
