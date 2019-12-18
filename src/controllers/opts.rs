use structopt::StructOpt;

use crate::models::{
    EntryType
};

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Opt {
    #[structopt(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Command {
    Add {
        #[structopt(subcommand)]
        entry_type: EntryType,
    },
    Complete,
    Cancel,
    Remove,
}