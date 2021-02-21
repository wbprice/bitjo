extern crate termion;
mod controllers;
mod models;
mod views;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::views::Application;

fn main() {
    let application = Application::new();
}
