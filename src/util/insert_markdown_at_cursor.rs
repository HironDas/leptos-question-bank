use leptos::html;
use leptos::prelude::*;

pub enum Syntex {
    Bold,
    Italic,
    Underscore,
    Dash,
    Bullet,
    Order,
    Heading1,
    Heading2,
    Heading3,
}

impl Syntex {
    fn as_str(&self) -> &str {
        match self {
            Self::Bold => "**",
            Self::Dash => "\\_\\_\\_\\_",
            Self::Italic => "_",
            Self::Underscore => "<ins>",
            Self::Bullet => "- ",
            Self::Order => "1. ",
            Self::Heading1 => "# ",
            Self::Heading2 => "## ",
            Self::Heading3 => "### ",
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
            match syntex {
                Syntex::Heading1
                | Syntex::Heading2
                | Syntex::Heading3
                | Syntex::Bullet
                | Syntex::Order => format!("{}", syntex.as_str()),
                Syntex::Underscore => format!("{}{}", "<ins>", "</ins>"),
                _ => format!("{}{}", syntex.as_str(), syntex.as_str()),
            }
        } else {
            match syntex {
                Syntex::Heading1
                | Syntex::Heading2
                | Syntex::Heading3
                | Syntex::Bullet
                | Syntex::Order => format!("{}{}", syntex.as_str(), selected_text),
                Syntex::Underscore => format!("{}{}{}", "<ins>", selected_text, "</ins>"),
                _ => format!("{}{}{}", syntex.as_str(), selected_text, syntex.as_str()),
            }
        };

        let new_value = format!("{}{}{}", &value[0..start], replacement, &value[end..]);
        textarea.set_value(&new_value);

        let _ = textarea.focus();
        let new_cursor_pos = if selected_text.is_empty() {
            match syntex {
                Syntex::Dash => (start + replacement.len()) as u32,
                _ => (start + syntex.as_str().to_string().len()) as u32,
            }
        } else {
            (start + replacement.len()) as u32
        };
        let _ = textarea.set_selection_range(new_cursor_pos, new_cursor_pos);
    }
}
