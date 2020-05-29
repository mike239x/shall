use std::io;
use std::io::Write;

use termion::screen::AlternateScreen;
// use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Clear};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Style, Color};

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut stdout = AlternateScreen::from(io::stdout().into_raw_mode().unwrap());
    // let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => break,
            Key::Char(c) => {
            },
            _ => {}
        }
        // stdout.flush().unwrap(); ?
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    vec![
                        Constraint::Min(1),
                        Constraint::Length(1),
                    ]
                )
                .split(f.size());
            let powerline = chunks[1];
            let rest   = chunks[0];
            let chunks =  Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    vec![
                        Constraint::Length(3),
                        Constraint::Min(1)
                    ]
                )
                .split(rest);
            let line_numbers = chunks[0];
            let text_buffer  = chunks[1];
            let block = Block::default()
                 .title("text buff")
                 .borders(Borders::ALL);
            f.render_widget(block, text_buffer);
            let block = Block::default()
                 .style(Style::default().bg(Color::Rgb(40,40,40)));
            f.render_widget(block, line_numbers);
            let block = Block::default()
                 .title("powerline")
                 .style(Style::default().bg(Color::Green));
            f.render_widget(block, powerline);
        });

    }
    Ok(())
}
