use crate::lib::entry::Entry;

#[derive(Default)]
pub struct Note {
    content: String,
    important: bool,
}

impl Entry for Note {
    fn text(&self) -> String {
        format!(
            "{important} {symbol} {content}",
            important = if self.important { "*" } else { " " },
            symbol = "-",
            content = self.content
        )
    }
}