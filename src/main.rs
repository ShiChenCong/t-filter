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

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    let app = Input::default();
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
    mut input: Input,
) -> Result<(), Box<dyn Error>> {
    let mut current_index = 0;

    let items = util::history::get_command_history().unwrap();
    // 这里的3就是下面的3行
    let page_size: usize = usize::from(terminal.size().unwrap().height) - 3;
    let mut current_page = 0;
    Ok(loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            // 3行
                            Constraint::Length(3),
                            // 这区域会占满剩余空间
                            Constraint::Min(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                f.render_widget(
                    Paragraph::new(input.value())
                        .style(Style::default().fg(Color::Yellow))
                        .block(Block::default().borders(Borders::ALL)),
                    chunks[0],
                );

                f.set_cursor(
                    // Put cursor past the end of the input text
                    chunks[0].x + (input.visual_cursor()) as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[0].y + 1,
                );

                // 如果input.value有值，则只显示筛选过的item
                let value = input.value();
                let contained_value_items: Vec<&String> =
                    items.iter().filter(|item| item.contains(value)).collect();
                let filtered_items = contained_value_items
                    .iter()
                    .map(|s| ListItem::new(s.to_string()))
                    .collect::<Vec<ListItem>>();
                let mut sliced_items = filtered_items
                    [current_page * page_size..current_page * page_size + page_size]
                    .to_vec();
                sliced_items[current_index] = sliced_items[current_index]
                    .clone()
                    .style(Style::default().bg(Color::Cyan).fg(Color::Black));

                f.render_widget(
                    List::new(sliced_items)
                        .block(Block::default())
                        .style(Style::default().fg(Color::White))
                        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                        .highlight_symbol(">>"),
                    chunks[1],
                );
            })
            .unwrap();
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        if current_index == 0 {
                            current_page -= 1;
                            current_index = page_size - 1;
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
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {
                        input.handle_event(&Event::Key(key));
                    }
                }
            }
        }
    })
}
