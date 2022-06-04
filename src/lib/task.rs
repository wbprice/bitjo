use crate::lib::entry::Entry;

#[derive(Default)]
pub struct Task {
    content: String,
    important: bool,
    completed: bool,
    children: Vec<Box<dyn Entry>>,
}

impl Task {
    pub fn new(content: String) -> Box<Self> {
        Box::new(Task {
            content,
            ..Default::default()
        })
    }
}

impl Entry for Task {
    fn text(&self) -> String {
        format!(
            "{important} {symbol} {content}",
            important = if self.important { "*" } else { " " },
            symbol = if self.completed { "X" } else { "\u{2022}" },
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
    fn it_creates_a_note() {
        let content = "Create a task".to_string();
        let task = Task::new(content.clone());
        assert!(task.text().contains(&content));
    }

    #[test]
    fn it_inserts_a_child_entry_to_a_task() {
        let parent = "Create a task".to_string();
        let child = "Create a child task".to_string();
        let mut task = Task::new(parent.clone());
        let note = Note::new(child.clone());
        task.insert(note);
        assert!(task.children().first().unwrap().text().contains(&child));
    }
}
