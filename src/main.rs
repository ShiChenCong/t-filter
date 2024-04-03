mod util;
use clipboard_rs::{Clipboard, ClipboardContext};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
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
    let res = run(&mut terminal, app).unwrap();

    restore_terminal(&mut terminal)?;
    let ctx = ClipboardContext::new().unwrap();
    ctx.set_text(res).unwrap();
    println!("已复制到粘贴板");
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
) -> Result<String, Box<dyn Error>> {
    let mut current_index = 0;
    let mut current_string = "".to_owned();

    let items = util::history::get_command_history().unwrap();
    // 这里的3就是下面的3行
    let page_size: usize = usize::from(terminal.size().unwrap().height) - 3;
    let mut current_page = 0;
    let res;
    let mut pre_value = String::new();
    loop {
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
                if value != pre_value.as_str() {
                    pre_value = value.to_string();
                    current_page = 0;
                }
                let contained_value_items: Vec<&String> =
                    items.iter().filter(|item| item.contains(value)).collect();
                // 创建ListItem
                let filtered_items = contained_value_items
                    .iter()
                    .map(|s| ListItem::new(s.to_string()))
                    .collect::<Vec<ListItem>>();
                // 只显示当前页的item
                let end_index = if filtered_items.len() == 0 {
                    0
                } else if filtered_items.len() < page_size {
                    filtered_items.len() - 1
                } else {
                    current_page * page_size + page_size
                };
                let mut sliced_items = filtered_items[current_page * page_size..end_index].to_vec();
                // 改变当前选中的item的背景色
                if sliced_items.len() > 0 {
                    current_string =
                        contained_value_items[current_page * page_size + current_index].to_owned();
                    sliced_items[current_index] = sliced_items[current_index]
                        .clone()
                        .style(Style::default().bg(Color::Cyan).fg(Color::Black));
                }

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
                    KeyCode::Char('n') if key.modifiers == KeyModifiers::CONTROL => {
                        current_index += 1;
                        if current_index == page_size {
                            current_page += 1;
                            current_index = 0;
                        }
                    }
                    KeyCode::Char('p') if key.modifiers == KeyModifiers::CONTROL => {
                        if current_index == 0 {
                            if current_page != 0 {
                                current_page -= 1;
                                current_index = page_size - 1;
                            }
                        } else {
                            current_index -= 1;
                        }
                    }
                    KeyCode::Up => {
                        if current_index == 0 {
                            if current_page != 0 {
                                current_page -= 1;
                                current_index = page_size - 1;
                            }
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
                    KeyCode::Enter => {
                        res = current_string;
                        break;
                    }
                    KeyCode::Esc => {
                        res = current_string;
                        break;
                    }
                    _ => {
                        input.handle_event(&Event::Key(key));
                    }
                }
            };
        };
    }
    Ok(res)
}
