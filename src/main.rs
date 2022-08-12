use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Frame,
    Terminal,
};
use crossterm::{
    event::{self, poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};

use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
}

fn main() -> Result<(), io::Error>  {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    println!("{}", type_of(&terminal));
    
    draw(&mut terminal);

    Resize(event: Event) {
        draw(&mut terminal);
    }

    thread::sleep(Duration::from_millis(5_000));
    draw(&mut terminal);
    thread::sleep(Duration::from_millis(5_000));


    // Terminal restoration
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn draw(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) {
    terminal.draw(|f| {
        ui(f);
    }).unwrap();
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10)
        ].as_ref())
        .split(f.size());
    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
        .title("Block 2")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
