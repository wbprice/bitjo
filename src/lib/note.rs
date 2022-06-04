use crate::lib::entry::Entry;

#[derive(Default)]
pub struct Note {
    content: String,
    important: bool,
    children: Vec<Box<dyn Entry>>,
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

    fn children(&self) -> &Vec<Box<dyn Entry>> {
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::Task;

    #[test]
    fn it_creates_a_note() {
        let content = "Create a note".to_string();
        let note = Note::new(content.clone());
        assert!(note.text().contains(&content));
    }

    #[test]
    fn it_inserts_a_child_entry_to_a_note() {
        let parent = "Create a note".to_string();
        let child = "Create a child task".to_string();
        let mut note = Note::new(parent.clone());
        let task = Task::new(child.clone());
        note.insert(task);
        assert!(note.children().first().unwrap().text().contains(&child));
    }
}
