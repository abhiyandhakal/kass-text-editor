use std::{cmp::Ordering, format, io::stdout, vec};

use crossterm::{cursor::SetCursorStyle, execute};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

use crate::{editor::Bound, enums::*, kass::Kass};

fn command_ui(kass: &mut Kass) -> Paragraph {
    let command_paragraph = Paragraph::new(Text::from(Spans::from(kass.app.command.clone())));
    let error_paragraph = Paragraph::new(Text::from(Spans::from(Span::styled(
        kass.app.error.clone(),
        Style::default()
            .fg(Color::Rgb(225, 130, 0))
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC),
    ))));
    let info_paragraph = Paragraph::new(Text::from(kass.app.info.clone()));

    match kass.app.action {
        Action::Command => command_paragraph,
        Action::Info => info_paragraph,
        Action::Error => error_paragraph,
    }
}

fn statusline_ui(kass: &mut Kass) -> Paragraph {
    let filepath = kass.app.tabs[kass.app.active_index].filepath.as_str();

    let filepath_span = Span::styled(filepath, Style::default().fg(Color::Black));

    let (statusline_span, style) = match kass.app.mode {
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

    Paragraph::new(statusline_text).style(Style::default().bg(Color::DarkGray))
}

fn tabs_ui(kass: &mut Kass) -> Tabs {
    let tab_titles = kass
        .app
        .tabs
        .iter()
        .enumerate()
        .map(|(i, tab)| {
            Spans::from(vec![Span::styled(
                format!(" {}. {} ", i, &tab.title),
                Style::default(),
            )])
        })
        .collect();

    Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(kass.app.active_index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Blue)
                .fg(Color::Black),
        )
}

fn editor_ui(kass: &mut Kass) -> (List, List) {
    // let (editor_width, editor_height) = kass.app.tabs[kass.app.active_index].boundary(terminal_width, terminal_height);
    let editor_width = kass.app.tabs[kass.app.active_index].editor_size.x;
    let editor_height = kass.app.tabs[kass.app.active_index].editor_size.y;

    // set bounds
    kass.app.tabs[kass.app.active_index].bounds = (
        Bound {
            x1: kass.app.tabs[kass.app.active_index].coloff,
            x2: kass.app.tabs[kass.app.active_index].coloff + editor_width,
        },
        Bound {
            x1: kass.app.tabs[kass.app.active_index].rowoff,
            x2: kass.app.tabs[kass.app.active_index].rowoff + editor_height,
        },
    );
    let (_bound_x, bound_y) = kass.app.tabs[kass.app.active_index].bounds.clone();

    let mut new_rows: Vec<String> = vec![];

    // vertical scrolling
    let mut i = 0;
    loop {
        let row = i;
        if i >= bound_y.x1 {
            new_rows.push(kass.app.tabs[kass.app.active_index].rows[row as usize].clone());
        }

        i += 1;

        if i as usize == kass.app.tabs[kass.app.active_index].rows.len() || i == bound_y.x2 {
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

    let line_numbers: Vec<ListItem> = match kass.line_number {
        LineNumber::Absolute => new_rows
            .iter()
            .enumerate()
            .map(|(i, _m)| {
                let number = vec![Spans::from(Span::raw(format!(
                    "{}",
                    i + 1 + kass.app.tabs[kass.app.active_index].rowoff as usize
                )))];
                ListItem::new(number)
            })
            .collect(),

        _ => kass.app.tabs[kass.app.active_index]
            .rows
            .iter()
            .enumerate()
            .map(|(i, _m)| {
                let row = i as u16 + kass.app.tabs[kass.app.active_index].rowoff;
                // Displays the relative line number
                let cursor_at = kass.app.tabs[kass.app.active_index].cursor.y
                    + kass.app.tabs[kass.app.active_index].rowoff;
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
            .collect(),
    };

    // update cursor
    kass.cursor.set_pos(
        kass.app.tabs[kass.app.active_index].cursor.x,
        kass.app.tabs[kass.app.active_index].cursor.y,
    );

    (
        List::new(rows).block(Block::default().borders(match kass.line_number {
            LineNumber::None => Borders::ALL,
            _ => Borders::RIGHT | Borders::TOP | Borders::BOTTOM,
        })), // rows
        List::new(line_numbers)
            .block(Block::default().borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)), // line_numbers
    )
}

pub fn ui<B: Backend>(kass: &mut Kass, frame: &mut Frame<B>) {
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

    frame.render_widget(statusline_ui(kass), chunks[2]);
    frame.render_widget(command_ui(kass), chunks[3]);
    frame.render_widget(tabs_ui(kass), chunks[0]);

    kass.app.action = Action::Command;

    // editor chunk
    let editor_chunk = Layout::default()
        .margin(0)
        .direction(tui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Length(match kass.line_number {
                LineNumber::None => 0,
                _ => 6,
            }),
            Constraint::Min(1),
        ])
        .split(chunks[1]);

    // editor height and width
    let editor_height = editor_chunk[1].height - 2;
    let editor_width = editor_chunk[1].width;
    kass.editor_size = (editor_width, editor_height);
    kass.app.tabs[kass.app.active_index].boundary(editor_width, editor_height); //setting height and width

    let (rows, line_numbers) = editor_ui(kass);

    frame.render_widget(line_numbers, editor_chunk[0]);
    frame.render_widget(rows, editor_chunk[1]);

    // cursor stuff
    match kass.app.mode {
        Mode::Insert => {
            execute!(stdout(), SetCursorStyle::BlinkingBar).expect("Couldn't enable blinking")
        }
        _ => execute!(stdout(), SetCursorStyle::SteadyBlock).expect("Couldn't disable blinking"),
    }

    match kass.app.mode {
        Mode::Command => {
            frame.set_cursor(chunks[2].x + kass.app.command.len() as u16, chunks[2].y + 1)
        }

        Mode::Normal => frame.set_cursor(
            if kass.cursor.x == 0 {
                match kass.line_number {
                    LineNumber::None => editor_chunk[1].x + 1,
                    _ => editor_chunk[1].x,
                }
            } else {
                match kass.line_number {
                    LineNumber::None => editor_chunk[1].x + kass.cursor.x + 1,
                    _ => editor_chunk[1].x + kass.cursor.x - 1,
                }
            },
            editor_chunk[1].y + kass.cursor.y + 1,
        ),

        _ => frame.set_cursor(
            editor_chunk[1].x + kass.cursor.x,
            editor_chunk[1].y + kass.cursor.y + 1,
        ),
    }
}
