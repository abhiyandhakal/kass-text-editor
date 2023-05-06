use std::{
    io::{stdout, Result},
    vec,
};

use crossterm::{
    cursor::SetCursorStyle,
    event::{self, Event, KeyCode},
    execute,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::enums::*;

struct App {
    mode: Mode,
    command: String,
    rows: Vec<String>,
}

impl App {
    fn new() -> App {
        App {
            mode: Mode::Normal,
            command: String::new(),
            rows: vec![],
        }
    }
}

pub struct Kass {
    app: App,
}

impl Kass {
    pub fn new() -> Result<Kass> {
        let app = App::new();
        Ok(Kass { app })
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                match self.app.mode {
                    Mode::Normal => match key.code {
                        event::KeyCode::Char(c) => match c {
                            'i' => self.app.mode = Mode::Insert,
                            ':' => {
                                self.app.mode = Mode::Command;
                                self.app.command.push(':');
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    Mode::Command => match key.code {
                        event::KeyCode::Char(ch) => match ch {
                            _ => self.app.command.push(ch),
                        },
                        KeyCode::Esc => {
                            self.app.mode = Mode::Normal;
                            self.app.command = String::new();
                        }
                        KeyCode::Enter => match self.app.command.as_str() {
                            ":q" => return Ok(()),
                            _ => {
                                self.app.mode = Mode::Normal;
                                self.app.command = String::new();
                            }
                        },
                        KeyCode::Backspace => {
                            if self.app.command.len() != 0 {
                                self.app.command.pop();
                            }
                        }
                        _ => {}
                    },
                    Mode::Insert => match key.code {
                        event::KeyCode::Char(c) => {
                            if let Some(last) = self.app.rows.last_mut() {
                                last.push(c)
                            } else {
                                self.app.rows.push(format!("{}", c));
                            }
                        }
                        event::KeyCode::Backspace => {
                            if let Some(last) = self.app.rows.last_mut() {
                                if last.len() != 0 {
                                    last.pop();
                                } else {
                                    if self.app.rows.len() != 1 {
                                        self.app.rows.pop();
                                    }
                                }
                            }
                        }
                        event::KeyCode::Enter => {
                            self.app.rows.push(String::new());
                        }
                        KeyCode::Tab => {
                            self.app.rows.remove(5);
                        }
                        event::KeyCode::Esc => self.app.mode = Mode::Normal,
                        _ => {}
                    },
                }
            }
        }
    }

    fn ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(frame.size());

        let (mode_span, style) = match self.app.mode {
            Mode::Normal => (
                vec![Span::raw("Normal")],
                Style::default().fg(Color::Yellow),
            ),
            Mode::Insert => (vec![Span::raw("Insert")], Style::default()),
            Mode::Command => (
                vec![Span::raw("Command")],
                Style::default().bg(Color::LightYellow).fg(Color::Black),
            ),
        };

        let mut mode_text = Text::from(Spans::from(mode_span));
        mode_text.patch_style(style);

        let mode_paragraph = Paragraph::new(mode_text).style(Style::default().bg(Color::DarkGray));
        frame.render_widget(mode_paragraph, chunks[1]);

        let command_paragraph = Paragraph::new(Text::from(Spans::from(self.app.command.clone())));
        frame.render_widget(command_paragraph, chunks[2]);

        let rows: Vec<ListItem> = self
            .app
            .rows
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{:<4}{}", i, m)))];
                ListItem::new(content)
            })
            .collect();

        let rows = List::new(rows).block(Block::default().borders(Borders::ALL));

        frame.render_widget(rows, chunks[0]);

        match self.app.mode {
            Mode::Command => {
                frame.set_cursor(chunks[2].x + self.app.command.len() as u16, chunks[2].y + 1)
            }
            _ => {
                if let Some(last) = self.app.rows.last_mut() {
                    frame.set_cursor(
                        chunks[0].x + last.len() as u16 + 1 + 4,
                        chunks[0].y + self.app.rows.len() as u16,
                    )
                }
            }
        }

        match self.app.mode {
            Mode::Insert => {
                execute!(stdout(), SetCursorStyle::BlinkingBar).expect("Couldn't enable blinking")
            }
            _ => {
                execute!(stdout(), SetCursorStyle::SteadyBlock).expect("Couldn't disable blinking")
            }
        }
    }
}
