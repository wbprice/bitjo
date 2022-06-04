use crate::lib::entry::Entry;

#[derive(Default)]
pub struct Event {
    content: String,
    important: bool,
    children: Vec<Box<dyn Entry>>,
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

    fn children(&self) -> &Vec<Box<dyn Entry>> {
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::Note;

    #[test]
    fn it_creates_an_event() {
        let content = "Create an event".to_string();
        let event = Event::new(content.clone());
        assert!(event.text().contains(&content));
    }

    #[test]
    fn it_inserts_a_child_entry_to_an_event() {
        let parent = "Create an event".to_string();
        let child = "Create a child note".to_string();
        let mut event = Event::new(parent.clone());
        let note = Note::new(child.clone());
        event.insert(note);
        assert!(event.children().first().unwrap().text().contains(&child));
    }
}
