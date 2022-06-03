use crate::lib::entry::Entry;

#[derive(Default)]
pub struct Event {
    content: String,
    important: bool,
}

impl Entry for Event {
    fn text(&self) -> String {
        format!(
            "{important} {symbol} {content}",
            important = if self.important { "*" } else { " " },
            symbol = "\u{26AC}",
            content = self.content
        )
    }
}