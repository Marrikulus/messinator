#[allow(dead_code)]
mod util;

use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Corner, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, SelectableList, Paragraph, Text, Widget};
use tui::Terminal;

use crate::util::event::{Event, Events, Config};

static fb_dstb: &'static str = "AQHms2WJ0vWc:AQGC0tS4cu6X";
static c_user: &'static str = "1138299783";


enum View {
    List,
    Chat(usize),
}

struct App<'a> {
    should_quit: bool,
    view: View,
    items: Vec<&'a str>,
    selected: Option<usize>,
    selectedItems: Vec<&'a str>,
    input: String,
    messages: Vec<String>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            should_quit: false,
            view: View::List,
            items: vec![
                "Dovy", "Ingveldur", "Guðmundur", "Andri", "Sigmar", "Jón", "Gestur", "Siggi", "Þórhildur",
            ],
            selected: None,
            selectedItems: vec![
                "test",
                "test2",
                "test3"
            ],
            input: String::new(),
            messages: Vec::new(),
        }
    }

    fn advance(&mut self) {

    }

    fn process_events(&mut self, input: Key) {

        match self.view {
            View::Chat(i) => {
                match input {
                    Key::Esc => {
                        self.view = View::List;
                    }
                    Key::Char('\n') => {
                        self.messages.push(self.input.drain(..).collect());
                    }
                    Key::Char(c) => {
                        self.input.push(c);
                    }
                    Key::Backspace => {
                        self.input.pop();
                    }
                    _ => {}
                }
            }
            View::List => {
                match input {
                    Key::Esc => {
                        self.should_quit = true;
                    }
                    Key::Char('r') => {
                        let event = self.selectedItems.pop().unwrap();
                        self.selectedItems.insert(0, event);
                    }
                    Key::Char('\n') => {
                        if let Some(selected) = self.selected {
                            self.view = View::Chat(selected);
                        }
                    }
                    Key::Left => {
                        self.selected = None;
                    }
                    Key::Down => {
                        self.selected = if let Some(selected) = self.selected {
                            if selected >= self.items.len() - 1 {
                                Some(0)
                            } else {
                                Some(selected + 1)
                            }
                        } else {
                            Some(0)
                        };
                    }
                    Key::Up => {
                        self.selected = if let Some(selected) = self.selected {
                            if selected > 0 {
                                Some(selected - 1)
                            } else {
                                Some(self.items.len() - 1)
                            }
                        } else {
                            Some(0)
                        };
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;


    let config = Config::default();
    let events = Events::with_config(config);

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|mut f| {
            match app.view {
                View::List => {
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                        .split(f.size());

                    let style = Style::default().fg(Color::Black).bg(Color::White);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("List"))
                        .items(&app.items)
                        .select(app.selected)
                        .style(style)
                        .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
                        .highlight_symbol(">")
                        .render(&mut f, chunks[0]);

                    {
                        let items = app.selectedItems.iter().map(|&item| {
                            Text::styled(item, style)
                        });

                        List::new(items)
                            .start_corner(Corner::BottomLeft)
                            .render(&mut f, chunks[1]);
                    }

                }
                View::Chat(i) => {
                    let name = app.items[i];
                    let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(2)
			.constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
			.split(f.size());

		    Paragraph::new([Text::raw(&app.input)].iter())
			.style(Style::default().fg(Color::Yellow))
			.block(Block::default().borders(Borders::ALL).title("Input"))
			.render(&mut f, chunks[0]);

		    let messages = app
			.messages
			.iter()
			.enumerate()
			.map(|(i, m)| Text::raw(format!("{}: {}", i, m)));

		    List::new(messages)
			.block(Block::default().borders(Borders::ALL).title(name))
			.render(&mut f, chunks[1]);
                }
            }
        })?;

        match events.next()? {
            Event::Input(input) => {
                app.process_events(input);
            },
            Event::Tick => {
                app.advance();
                if app.should_quit {
                    break;
                }
            }
        }
    }

    Ok(())
}
