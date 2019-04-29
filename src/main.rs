use termion::{color, clear};

fn main() {
    println!("{}", clear::All);
    println!("{green}Bullet Journal CLI v0.1.0{reset}",
           green = color::Fg(color::Green),
           reset = color::Fg(color::Reset));
}