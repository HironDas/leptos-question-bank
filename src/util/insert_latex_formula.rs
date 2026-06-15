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

        let start = textarea.selection_start().unwrap_or(None).unwrap_or(0) as usize;
        let end = textarea.selection_end().unwrap_or(None).unwrap_or(0) as usize;

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
            // _ => format!(""),
        };

        let new_value = format!("{}{}{}", &value[0..start], replacement, &value[end..]);
        textarea.set_value(&new_value);
        let _ = textarea.focus();
        let new_cursor_pos = (start + replacement.len()) as u32;

        let _ = textarea.set_selection_range(new_cursor_pos, new_cursor_pos);
    }
}
