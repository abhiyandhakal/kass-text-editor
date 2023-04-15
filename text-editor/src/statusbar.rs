use std::io::{stdout, Result};

use crossterm::{
    cursor, queue,
    style::{self, Stylize},
};

#[derive(Debug, Clone)]
pub struct Statusbar {
    mode: String,
    terminal_height: usize,
    terminal_width: usize,
}

impl Statusbar {
    pub fn new(mode: String, terminal_height: usize, terminal_width: usize) -> Result<Statusbar> {
        Ok(Statusbar {
            mode,
            terminal_width,
            terminal_height,
        })
    }

    pub fn paint(&self) -> Result<()> {
        for x in 0..self.terminal_width {
            queue!(
                stdout(),
                cursor::MoveTo(x as u16, self.terminal_height as u16),
                style::PrintStyledContent("█".magenta())
            )?;
        }

        Ok(())
    }
}
