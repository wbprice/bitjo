mod lib;

use crate::lib::{Entry, Note, Event, Task};

fn main() {
    let entries: Vec<Box<dyn Entry>> = vec![
        Box::new(Note {
            content: "Hello note!".into(),
            ..Note::default()
        }),
        Box::new(Event {
            content: "Hello event!".into(),
            ..Event::default()
        }),
        Box::new(Task {
            content: "Hello todo!".into(),
            ..Task::default()
        }),
    ];

    for entry in entries {
        println!("{}", entry.text());
    }
}
