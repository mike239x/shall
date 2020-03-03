use termion::event::Key;
use termion::input::TermRead;
// use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{Write, stdout, stdin};

/// a window on the terminal screen
#[allow(dead_code)]
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

    // shorthands for common actions
    macro_rules! clear_screen {
        () => { print!("{}", termion::clear::All); };
    }
    macro_rules! go_to {
        ($x:expr, $y:expr) => {
            print!("{}", termion::cursor::Goto($x, $y));
        };
    }
    macro_rules! bg {
        ($r:expr, $g:expr, $b:expr) => {
            print!("{}", termion::color::Bg(termion::color::Rgb($r, $g, $b)));
        };
    }
    macro_rules! fg {
        ($r:expr, $g:expr, $b:expr) => {
            print!("{}", termion::color::Fg(termion::color::Rgb($r, $g, $b)));
        };
    }

    bg!( 20,  20,  20);
    fg!(250, 250, 250);

    clear_screen!();

    for k in 1..=terminal_size.1 {
        go_to!(1, k);
        print!("{}", k);
    }

    print!(" {:?}", terminal_size);

    go_to!(1, 1);

    stdout.flush().unwrap();

    let _powerline = Window {
        x: 1,
        y: terminal_size.1,
        w: terminal_size.0,
        h: 1,
    };

    let _line_numbers = Window {
        x: 1,
        y: 1,
        w: 3,
        h: terminal_size.1 - 1,
    };

    let _text_buffer = Window {
        x: 4,
        y: 1,
        w: terminal_size.0 - 3,
        h: terminal_size.1 - 1,
    };

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
