mod util;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use std::{
    error::Error,
    io::{self, Stdout},
    time::Duration,
};
fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    run(&mut terminal)?;

    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut current_index = 0;

    Ok(loop {
        terminal
            .draw(|f| {
                let size = f.size();
                let mut items = util::history::get_command_history()
                    .unwrap()
                    .iter()
                    .map(|s| ListItem::new(s.clone()))
                    .collect::<Vec<ListItem>>();
                items[current_index] = ListItem::new("Item 4")
                    .style(Style::default().bg(Color::Cyan).fg(Color::Black));

                f.render_widget(
                    List::new(items)
                        .block(Block::default().title("List").borders(Borders::ALL))
                        .style(Style::default().fg(Color::White))
                        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                        .highlight_symbol(">>"),
                    size,
                );
            })
            .unwrap();
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        current_index -= 1;
                    }
                    KeyCode::Down => {
                        current_index += 1;
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    })
}
