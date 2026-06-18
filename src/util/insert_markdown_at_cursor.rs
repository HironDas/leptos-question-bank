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
    Tally(u32),
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
            Self::Formula => "\n$$\n",
            Self::Tally(val) => match val {
                1 => ":tally1:",
                2 => ":tally2:",
                3 => ":tally3:",
                4 => ":tally4:",
                5 => ":tally5:",
                _ => ":tally5:", // Fallback default
            },
            _ => "",
        }
    }
}

pub fn insert_markdown(textarea_ref: NodeRef<html::Textarea>, syntex: Syntex) {
    if let Some(textarea) = textarea_ref.get_untracked() {
        let value = textarea.value();

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
                break; // We found both boundary edges, safe to terminate loop early
            }
        }

        // 3. Slice using calculated byte-safe boundaries
        let selected_text = &value[byte_start..byte_end];

        let replacement = if selected_text.is_empty() {
            match syntex {
                Syntex::Heading1
                | Syntex::Heading2
                | Syntex::Heading3
                | Syntex::Bullet
                | Syntex::Order
                | Syntex::Tally(_) => format!("{}", syntex.as_str()),
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
                | Syntex::Order
                | Syntex::Tally(_) => format!("{}{}", syntex.as_str(), selected_text),
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

        let new_value = format!(
            "{}{}{}",
            &value[0..byte_start],
            replacement,
            &value[byte_end..]
        );
        textarea.set_value(&new_value);

        let _ = textarea.focus();
        // 5. Calculate new cursor coordinates using character lengths (since JS needs characters)
        let replacement_char_len = replacement.chars().count();
        let syntex_char_len = syntex.as_str().chars().count();

        let new_cursor_pos = if selected_text.is_empty() {
            match syntex {
                Syntex::Dash => (char_start + replacement_char_len) as u32,
                _ => (char_start + syntex_char_len) as u32,
            }
        } else {
            (char_start + replacement_char_len) as u32
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
