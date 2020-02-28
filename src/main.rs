// use termion::{color, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

     write!(stdout,
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
            .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('\n') => print!("\n\r"),
            Key::Char(c) => print!("{}", c),
            Key::Alt(c) => print!("^{}", c),
            Key::Ctrl(c) => print!("*{}", c),
            Key::Esc => print!("ESC"),
            Key::Left => print!("←"),
            Key::Right => print!("→"),
            Key::Up => print!("↑"),
            Key::Down => print!("↓"),
            Key::Backspace => print!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),                                   
        termion::cursor::Show
    ).unwrap();
}
