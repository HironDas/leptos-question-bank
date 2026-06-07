use leptos::html;
use leptos::prelude::*;

pub enum Syntex {
    Bold,
    Italic,
    Underscore,
    Dash,
}

impl Syntex {
    fn as_str(&self) -> &str {
        match self {
            Self::Bold => "**",
            Self::Dash => "\\_\\_\\_\\_",
            Self::Italic => "_",
            Self::Underscore => "<ins>",
        }
    }
}

pub fn insert_markdown(textarea_ref: NodeRef<html::Textarea>, syntex: Syntex) {
    if let Some(textarea) = textarea_ref.get_untracked() {
        let value = textarea.value();

        let start = textarea.selection_start().unwrap_or(None).unwrap_or(0) as usize;
        let end = textarea.selection_end().unwrap_or(None).unwrap_or(0) as usize;

        let selected_text = &value[start..end];

        let replacement = if selected_text.is_empty() {
            if syntex.as_str() == "<ins>" {
                format!("{}{}", "<ins>", "</ins>")
            } else {
                format!("{}{}", syntex.as_str(), syntex.as_str())
            }
        } else {
            if syntex.as_str() == "<ins>" {
                format!("{}{}{}", "<ins>", selected_text, "</ins>")
            } else {
                format!("{}{}{}", syntex.as_str(), selected_text, syntex.as_str())
            }
        };

        let new_value = format!("{}{}{}", &value[0..start], replacement, &value[end..]);
        textarea.set_value(&new_value);

        let _ = textarea.focus();
        let new_cursor_pos = (start + replacement.len()) as u32;
        let _ = textarea.set_selection_range(new_cursor_pos, new_cursor_pos);
    }
}
