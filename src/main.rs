mod util;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::{
    error::Error,
    io::{self, Stdout},
    time::Duration,
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

struct App {
    /// Current value of the input box
    input: Input,
    /// History of recorded messages
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    let app = App::default();
    run(&mut terminal, app)?;

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

fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
) -> Result<(), Box<dyn Error>> {
    let mut current_index = 0;

    let items = util::history::get_command_history()
        .unwrap()
        .iter()
        .map(|s| ListItem::new(s.clone()))
        .collect::<Vec<ListItem>>();
    let page_size: usize = usize::from(terminal.size().unwrap().height) - 2;
    let mut current_page = 0;
    Ok(loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(1),
                            Constraint::Length(3),
                            Constraint::Min(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                let mut cloned_items =
                    items[current_page * page_size..current_page * page_size + page_size].to_vec();
                cloned_items[current_index] = cloned_items[current_index]
                    .clone()
                    .style(Style::default().bg(Color::Cyan).fg(Color::Black));

                let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

                let scroll = app.input.visual_scroll(width as usize);
                let input = Paragraph::new(app.input.value())
                    .style(Style::default().fg(Color::Yellow))
                    .scroll((0, scroll as u16))
                    .block(Block::default().borders(Borders::ALL).title("Input"));
                f.render_widget(input, chunks[1]);
                f.set_cursor(
                    // Put cursor past the end of the input text
                    chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[1].y + 1,
                )
            })
            .unwrap();
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        if current_index == 0 {
                        } else {
                            current_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        current_index += 1;
                        if current_index == page_size {
                            current_page += 1;
                            current_index = 0;
                        }
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {
                        app.input.handle_event(&Event::Key(key));
                    }
                }
            }
        }
    })
}
