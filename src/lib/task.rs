use crate::lib::entry::Entry;

#[derive(Default)]
pub struct Task {
    content: String,
    important: bool,
    completed: bool,
    children: Vec<Box<dyn Entry>>
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
}
