// use termion::{color, style};
use termion::event::Key;
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{Write, stdout, stdin};

/// a window on the terminal screen
struct Window {
    x : u16,
    y : u16,
    w : u16,
    h : u16,
}

fn main() {
    let stdin = stdin();
    // let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(stdout, "{}", termion::cursor::Show).unwrap();

    let terminal_size = termion::terminal_size().unwrap();

    macro_rules! clear_screen {
        () => {
            print!("{}", termion::clear::All);
        };
    }
    macro_rules! go_to {
        ($x:expr, $y:expr) => {
            print!("{}", termion::cursor::Goto($x, $y));
        };
    }

    clear_screen!();
    for k in 1..=terminal_size.1 {
        go_to!(1, k);
        print!("{}", k);
    }

    print!(" {:?}", terminal_size);

    go_to!(1, 1);

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => print!("\n\r"),
            Key::Char(c) => print!("{}", c),
            Key::Esc => break,
            Key::Backspace => print!("\u{8} \u{8}"),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    clear_screen!();
    go_to!(1, 1);
}
