trait Entry {
    fn text(&self) -> &String;
}

struct Note {
    content: String,
}

impl Entry for Note {
    fn text(&self) -> &String {
        &self.content
    }
}

struct Event {
    content: String,
}

impl Entry for Event {
    fn text(&self) -> &String {
        &self.content
    }
}

struct Todo {
    content: String,
}

impl Entry for Todo {
    fn text(&self) -> &String {
        &self.content
    }
}

fn main() {
    let entries: Vec<Box<dyn Entry>> = vec![
        Box::new(Note { content: "Hello note!".into() }),
        Box::new(Event { content: "Hello note!".into() }),
        Box::new(Todo { content: "Hello todo!".into() })
    ];

    for entry in entries {
        println!("{}", entry.text());
    }
}
