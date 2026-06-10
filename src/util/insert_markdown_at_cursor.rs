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
    Formula,
    Table(u32, u32),
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
            Self::Formula => "$$",
            _ => "",
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
                Syntex::Table(rows, columns) => format!("{}", tables_str(rows, columns)),
                _ => format!("{}{}", syntex.as_str(), syntex.as_str()),
            }
        } else {
            match syntex {
                Syntex::Heading1
                | Syntex::Heading2
                | Syntex::Heading3
                | Syntex::Bullet
                | Syntex::Order => format!("{}{}", syntex.as_str(), selected_text),
                Syntex::Table(rows, columns) => {
                    format!(
                        "{}\n\
                        {}",
                        selected_text,
                        tables_str(rows, columns)
                    )
                }
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

pub fn tables_str(rows: u32, columns: u32) -> String {
    let mut table = String::from("");
    for row in 0..(rows + 2) {
        table.push_str("|");
        for _column in 0..columns {
            let row_str = match row {
                0 => " Header |",
                1 => "--------|",
                _ => "    cell    |",
            };
            table.push_str(row_str);
        }
        table.push_str("\n");
    }

    table
}
