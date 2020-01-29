use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum EntryOpts {
    /// Add a new task entry
    Task {
        /// The task description
        text: String,
    },
    /// Add a new note entry
    Note {
        /// The note contents
        text: String,
    },
    /// Add a new event entry
    Event {
        /// The event description
        text: String,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Opt {
    #[structopt(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Command {
    /// Adds a new entry of a given type to the journal
    Add {
        #[structopt(subcommand)]
        new_entry: EntryOpts,
    },
    /// Toggles the importance of the nth entry in the list
    Emph { index: usize },
    /// Toggles the completion of the nth entry in the list
    Complete { index: usize },
    /// Removes the nth entry in the list
    Remove { index: usize },
    /// Allows for nesting of entries
    Sub {
        index: usize,
        #[structopt(subcommand)]
        command: Option<Box<Command>>
    }
}
