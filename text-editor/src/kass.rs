use std::{
    io::{stdout, Result},
    vec,
};

use crossterm::{
    cursor::SetCursorStyle,
    event::{self, Event, KeyEvent, KeyEventState, KeyModifiers},
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

use crate::{
    editor::{Bound, Editor},
    enums::*,
    mode_handlers::{
        command::handle_command_mode, insert::handle_insert_mode, normal::handle_normal_mode,
    },
    position::Position,
};

pub struct App {
    pub mode: Mode,
    pub tabs: Vec<Editor>,
    pub command: String,
    pub clipboard: Vec<String>,
    pub active_index: usize,
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
    pub app: App,
    pub key_event: KeyEvent,
    pub cursor: Position,
    pub editor_size: (u16, u16),
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
            editor_size: (0, 0),
        })
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let mut close = false;

        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                self.key_event = key;

                match self.app.mode {
                    Mode::Normal => handle_normal_mode(self)?,
                    Mode::Command => handle_command_mode(self, &mut close)?,
                    Mode::Insert => handle_insert_mode(self)?,
                }
            }

            if close {
                break;
            }
        }

        Ok(())
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

        // editor height and width
        let editor_height = chunks[1].height - 2;
        let editor_width = chunks[1].width;
        self.editor_size = (editor_width, editor_height);

        // set bounds
        self.app.tabs[self.app.active_index].bounds = (
            Bound {
                x1: self.app.tabs[self.app.active_index].coloff,
                x2: self.app.tabs[self.app.active_index].coloff + editor_width,
            },
            Bound {
                x1: self.app.tabs[self.app.active_index].rowoff,
                x2: self.app.tabs[self.app.active_index].rowoff + editor_height,
            },
        );
        let (bound_x, bound_y) = self.app.tabs[self.app.active_index].bounds.clone();

        let mut new_rows: Vec<String> = vec![];

        // vertical scrolling
        let mut i = 0;
        loop {
            let row = i;
            if i >= bound_y.x1 {
                new_rows.push(self.app.tabs[self.app.active_index].rows[row as usize].clone());
            }

            i += 1;

            if i as usize == self.app.tabs[self.app.active_index].rows.len() || i == bound_y.x2 {
                break;
            }
        }

        let rows: Vec<ListItem> = new_rows
            .iter()
            .enumerate()
            .map(|(_i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}", m)))];
                ListItem::new(content)
            })
            .collect();

        let rows = List::new(rows).block(Block::default().borders(Borders::ALL));

        // update cursor

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
