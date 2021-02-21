use std::io::{stdin, stdout, Write};
use termion::raw::IntoRawMode;

use crate::views::{header_bar::HeaderBar, text_area::TextArea};

pub struct Application;

impl Application {
    pub fn new() {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(stdout, "{}", termion::clear::All).unwrap();
        let header_bar = HeaderBar;
        let mut text_area = TextArea::new();

        header_bar.render(&mut stdout);
        text_area.handle_input(stdin, &mut stdout);

        // Show the cursor again before we exit.
        write!(stdout, "{}", termion::cursor::Show).unwrap();
    }
}
