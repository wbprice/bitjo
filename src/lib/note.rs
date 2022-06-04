use crate::lib::entry::Entry;

#[derive(Default)]
pub struct Note {
    content: String,
    important: bool,
    children: Vec<Box<dyn Entry>>
}

impl Note {
    pub fn new(content: String) -> Box<Self> {
        Box::new(Note {
            content,
            ..Default::default()
        })
    }
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

    fn insert(&mut self, entry: Box<dyn Entry>) {
        self.children.push(entry);
    }
}
