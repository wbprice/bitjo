extern crate termion;
mod models;
mod views;
mod controllers;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

use crate::views::Application;

fn main() {
   let application = Application::new();
}