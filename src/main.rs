mod lib;

use crate::lib::{Entry, Note, Event, Task};

fn main() {
    let entries: Vec<Box<dyn Entry>> = vec![
        Box::new(Note::new("Hello note!".into())),
        Box::new(Event::new("Hello event!".into())),
        Box::new(Task::new("Hello todo!".into()))
    ];

    for entry in entries {
        println!("{}", entry.text());
    }
}
