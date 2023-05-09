use std::{
    io::{stdout, Result},
    vec,
};

use crossterm::{
    cursor::SetCursorStyle,
    event::{self, Event, KeyCode, KeyEvent, KeyEventState, KeyModifiers},
    execute,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::{editor::Editor, enums::*, position::Position};

struct App {
    mode: Mode,
    tabs: Vec<Editor>,
    command: String,
    clipboard: Vec<String>,
    active_index: usize,
}

impl App {
    fn new() -> Result<App> {
        Ok(App {
            mode: Mode::Normal,
            command: String::new(),
            tabs: vec![Editor::new()],
            clipboard: vec![],
            active_index: 0,
        })
    }
    pub fn next(&mut self) {
        self.active_index = (self.active_index + 1) % self.tabs.len();
    }

    pub fn previous(&mut self) {
        if self.active_index > 0 {
            self.active_index -= 1;
        } else {
            self.active_index = self.tabs.len() - 1;
        }
    }
}

pub struct Kass {
    app: App,
    key_event: KeyEvent,
    cursor: Position,
}

impl Kass {
    pub fn new() -> Result<Kass> {
        let app = App::new()?;
        Ok(Kass {
            app,
            key_event: KeyEvent {
                code: crossterm::event::KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                kind: crossterm::event::KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            cursor: Position::new(),
        })
    }

    fn handle_normal_mode(&mut self) -> Result<()> {
        match self.key_event {
            KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => match c {
                'i' => {
                    self.app.tabs[self.app.active_index].move_left(1);
                    self.app.mode = Mode::Insert;
                }
                'a' => {
                    self.app.mode = Mode::Insert;
                }
                ':' => {
                    self.app.mode = Mode::Command;
                    self.app.command.push(':');
                }
                // navigation
                'l' => self.app.tabs[self.app.active_index].move_right(1),
                'h' => self.app.tabs[self.app.active_index].move_left(1),
                'j' => self.app.tabs[self.app.active_index].move_down(1),
                'k' => self.app.tabs[self.app.active_index].move_up(1),
                _ => {}
            },
            KeyEvent {
                code: event::KeyCode::Tab,
                ..
            } => self.app.next(),
            KeyEvent {
                code: event::KeyCode::BackTab,
                ..
            } => self.app.previous(),
            _ => {}
        }

        Ok(())
    }

    fn handle_insert_mode(&mut self) -> Result<()> {
        match self.key_event.code {
            event::KeyCode::Char(c) => {
                if self.cursor.x as usize
                    == self.app.tabs[self.app.active_index].rows[self.cursor.y as usize].len()
                    || self.app.tabs[self.app.active_index].rows[self.cursor.y as usize].len() == 0
                {
                    self.app.tabs[self.app.active_index].rows[self.cursor.y as usize].push(c);
                } else {
                    self.app.tabs[self.app.active_index].rows[self.cursor.y as usize]
                        .insert(self.cursor.x as usize, c);
                }

                self.app.tabs[self.app.active_index].move_right(1);
            }
            event::KeyCode::Backspace => {
                if let Some(last) = self.app.tabs[self.app.active_index].rows.last_mut() {
                    if last.len() != 0 {
                        last.pop();
                        self.app.tabs[self.app.active_index].move_left(1);
                    } else {
                        if self.app.tabs[self.app.active_index].rows.len() != 1 {
                            self.app.tabs[self.app.active_index].rows.pop();
                        }
                    }
                }
            }
            event::KeyCode::Enter => {
                self.app.tabs[self.app.active_index]
                    .rows
                    .push(String::new());

                self.app.tabs[self.app.active_index].move_down(1);
            }
            event::KeyCode::Esc => {
                self.app.mode = Mode::Normal;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                self.key_event = key;

                match self.app.mode {
                    Mode::Normal => self.handle_normal_mode()?,
                    Mode::Command => match key.code {
                        event::KeyCode::Char(ch) => match ch {
                            _ => self.app.command.push(ch),
                        },
                        KeyCode::Esc => {
                            self.app.mode = Mode::Normal;
                            self.app.command = String::new();
                        }
                        KeyCode::Enter => {
                            match self.app.command.as_str() {
                                ":q" => return Ok(()),
                                ":tabnew" => {
                                    self.app.tabs.push(Editor::new());
                                    self.app.active_index = self.app.tabs.len() - 1;
                                }
                                _ => {
                                    self.app.mode = Mode::Normal;
                                    self.app.command = String::new();
                                }
                            };

                            self.app.mode = Mode::Normal;
                            self.app.command = String::new();
                        }
                        KeyCode::Backspace => {
                            if self.app.command.len() != 0 {
                                self.app.command.pop();
                            }
                        }
                        _ => {}
                    },
                    Mode::Insert => self.handle_insert_mode()?,
                }
            }
        }
    }

    fn ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(3),
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
            Mode::Command => (vec![Span::raw("Command")], Style::default()),
        };

        let mut mode_text = Text::from(Spans::from(mode_span));
        mode_text.patch_style(style);

        let mode_paragraph = Paragraph::new(mode_text).style(Style::default().bg(Color::DarkGray));
        frame.render_widget(mode_paragraph, chunks[2]);

        let command_paragraph = Paragraph::new(Text::from(Spans::from(self.app.command.clone())));
        frame.render_widget(command_paragraph, chunks[3]);

        let tab_titles = self
            .app
            .tabs
            .iter()
            .map(|_tab| Spans::from(vec![Span::styled("abc", style)]))
            .collect();

        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.app.active_index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Red));

        frame.render_widget(tabs, chunks[0]);

        let rows: Vec<ListItem> = self.app.tabs[self.app.active_index]
            .rows
            .iter()
            .enumerate()
            .map(|(_i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}", m)))];
                ListItem::new(content)
            })
            .collect();

        let rows = List::new(rows).block(Block::default().borders(Borders::ALL));

        self.cursor.set_pos(
            self.app.tabs[self.app.active_index].cursor.x,
            self.app.tabs[self.app.active_index].cursor.y,
        );

        frame.render_widget(rows, chunks[1]);

        // cursor stuff
        match self.app.mode {
            Mode::Insert => {
                execute!(stdout(), SetCursorStyle::BlinkingBar).expect("Couldn't enable blinking")
            }
            _ => {
                execute!(stdout(), SetCursorStyle::SteadyBlock).expect("Couldn't disable blinking")
            }
        }

        match self.app.mode {
            Mode::Command => {
                frame.set_cursor(chunks[2].x + self.app.command.len() as u16, chunks[2].y + 1)
            }

            Mode::Normal => frame.set_cursor(
                if self.cursor.x == 0 {
                    chunks[1].x + 1
                } else {
                    chunks[1].x + self.cursor.x
                },
                chunks[1].y + self.cursor.y + 1,
            ),

            _ => frame.set_cursor(
                chunks[1].x + self.cursor.x + 1,
                chunks[1].y + self.cursor.y + 1,
            ),
        }
    }
}
