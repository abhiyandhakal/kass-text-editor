use std::{
    cmp::Ordering,
    env::current_dir,
    format,
    io::{stdout, Result},
    path::Path,
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
    pub error: String,
    pub info: String,

    pub action: Action,
    pub clipboard: Vec<String>,
    pub active_index: usize,
}

impl App {
    fn new() -> Result<App> {
        let mut filepath = "unnamed".to_string();
        let mut counter = 0;

        while Path::new(&filepath).exists() {
            counter += 1;
            filepath = format!("{}-{}", filepath, counter);
        }

        Ok(App {
            mode: Mode::Normal,
            command: String::new(),
            tabs: vec![Editor::new(filepath.clone())?],
            clipboard: vec![],
            active_index: 0,
            error: String::new(),
            info: String::new(),
            action: Action::Command,
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

        let filepath = format!(
            "{}/{}",
            current_dir()
                .expect("Couldn't get current dir")
                .to_str()
                .expect("Couldn't convert to string"),
            self.app.tabs[self.app.active_index].filepath.as_str()
        );

        let filepath_span = Span::styled(filepath, Style::default().fg(Color::Black));

        let (statusline_span, style) = match self.app.mode {
            Mode::Normal => (
                vec![
                    Span::styled("Normal", Style::default().fg(Color::Yellow)),
                    Span::raw("    "),
                    filepath_span,
                ],
                Style::default(),
            ),
            Mode::Insert => (
                vec![Span::raw("Insert"), Span::raw("    "), filepath_span],
                Style::default(),
            ),
            Mode::Command => (
                vec![Span::raw("Command"), Span::raw("    "), filepath_span],
                Style::default(),
            ),
        };

        let mut statusline_text = Text::from(Spans::from(statusline_span));
        statusline_text.patch_style(style);

        let statusline_paragraph =
            Paragraph::new(statusline_text).style(Style::default().bg(Color::DarkGray));
        frame.render_widget(statusline_paragraph, chunks[2]);

        let command_paragraph = Paragraph::new(Text::from(Spans::from(self.app.command.clone())));
        let error_paragraph = Paragraph::new(Text::from(Spans::from(Span::styled(
            self.app.error.clone(),
            Style::default()
                .fg(Color::Rgb(225, 130, 0))
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC),
        ))));
        let info_paragraph = Paragraph::new(Text::from(self.app.info.clone()));
        frame.render_widget(
            match self.app.action {
                Action::Command => command_paragraph,
                Action::Info => info_paragraph,
                Action::Error => error_paragraph,
            },
            chunks[3],
        );
        self.app.action = Action::Command;

        let tab_titles = self
            .app
            .tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| {
                Spans::from(vec![Span::styled(
                    format!(" {}. {} ", i, &tab.title),
                    style,
                )])
            })
            .collect();

        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.app.active_index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Blue)
                    .fg(Color::Black),
            );

        frame.render_widget(tabs, chunks[0]);

        // editor chunk
        let editor_chunk = Layout::default()
            .margin(0)
            .direction(tui::layout::Direction::Horizontal)
            .constraints([Constraint::Length(6), Constraint::Min(1)].as_ref())
            .split(chunks[1]);

        // editor height and width
        let editor_height = editor_chunk[1].height - 2;
        let editor_width = editor_chunk[1].width;
        self.editor_size = (editor_width, editor_height);
        self.app.tabs[self.app.active_index].boundary(editor_width, editor_height); //setting height and width

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
        let (_bound_x, bound_y) = self.app.tabs[self.app.active_index].bounds.clone();

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

        // for displaying content of the editor
        let rows: Vec<ListItem> = new_rows
            .iter()
            .enumerate()
            .map(|(_i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}", m)))];
                ListItem::new(content)
            })
            .collect();

        let line_numbers: Vec<ListItem> = self.app.tabs[self.app.active_index]
            .rows
            .iter()
            .enumerate()
            .map(|(i, _m)| {
                let row = i as u16 + self.app.tabs[self.app.active_index].rowoff;
                // Displays the relative line number
                let cursor_at = self.app.tabs[self.app.active_index].cursor.y
                    + self.app.tabs[self.app.active_index].rowoff;
                let line_order = cursor_at.cmp(&row);

                let relative_ln = match line_order {
                    Ordering::Equal => row + 1,
                    Ordering::Greater => cursor_at - row,
                    Ordering::Less => row - cursor_at,
                };

                let number = vec![Spans::from(Span::styled(
                    if line_order == Ordering::Equal {
                        format!("{:<4}", relative_ln)
                    } else {
                        format!("{:4}", relative_ln)
                    },
                    Style::default().fg(Color::DarkGray),
                ))];

                ListItem::new(number)
            })
            .collect();

        let rows = List::new(rows)
            .block(Block::default().borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM));
        let line_numbers = List::new(line_numbers)
            .block(Block::default().borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM));

        // update cursor

        self.cursor.set_pos(
            self.app.tabs[self.app.active_index].cursor.x,
            self.app.tabs[self.app.active_index].cursor.y,
        );

        frame.render_widget(line_numbers, editor_chunk[0]);
        frame.render_widget(rows, editor_chunk[1]);

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
                    editor_chunk[1].x
                } else {
                    editor_chunk[1].x + self.cursor.x - 1
                },
                editor_chunk[1].y + self.cursor.y + 1,
            ),

            _ => frame.set_cursor(
                editor_chunk[1].x + self.cursor.x,
                editor_chunk[1].y + self.cursor.y + 1,
            ),
        }
    }

    pub fn set_info(&mut self, info: &str) {
        self.app.action = Action::Info;
        self.app.info = info.to_string();
    }

    pub fn set_error(&mut self, error: &str) {
        self.app.action = Action::Error;
        self.app.error = error.to_string();
    }
}
