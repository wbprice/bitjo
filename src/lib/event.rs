use crate::lib::entry::Entry;

#[derive(Default)]
pub struct Event {
    content: String,
    important: bool,
    children: Vec<Box<dyn Entry>>
}

impl Event {
    pub fn new(content: String) -> Box<Self> {
        Box::new(Event {
            content,
            ..Default::default()
        })
    }
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

    fn insert(&mut self, entry: Box<dyn Entry>) {
        self.children.push(entry);
    }
}
