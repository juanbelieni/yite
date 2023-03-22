use crate::{cursor::Cursor, Position, pos};
use crossterm::{
  cursor::MoveTo,
  execute, queue,
  terminal::{size, Clear, ClearType},
  Result, style::{Colors, SetColors, Color, ResetColor},
};
use std::io::{stdout, Stdout, Write};

pub struct Screen {
  pub stdout: Stdout,
  pub cursor: Cursor,
  pub pos: Position,
}

impl Screen {
  pub fn new() -> Self {
    Self {
      stdout: stdout(),
      cursor: Cursor::new(),
      pos: pos!(0, 0),
    }
  }

  pub fn size(&self) -> Result<(u16, u16)> {
    let (width, height) = size()?;
    Ok((width, height - 1))
  }

  pub fn clear(&mut self) -> Result<()> {
    execute!(self.stdout, Clear(ClearType::All))
  }

  pub fn write(&mut self, pos: (u16, u16), text: String) -> Result<()> {
    queue!(self.stdout, MoveTo(pos.0, pos.1 + 1))?;
    write!(self.stdout, "{}", text)
  }

  pub fn draw_cursor(&mut self) -> Result<()> {
    let x = self.cursor.pos.x - self.pos.x;
    let y = self.cursor.pos.y - self.pos.y;
    queue!(self.stdout, MoveTo(x, y + 1))
  }

  pub fn draw_status_bar(&mut self) -> Result<()> {
    let (width, _) = self.size()?;

    execute!(self.stdout, SetColors(Colors::new(Color::Black, Color::Grey)))?;

    for x in 0..width {
      queue!(self.stdout, MoveTo(x, 0))?;
      write!(self.stdout, " ")?;
    }

    queue!(self.stdout, MoveTo(0, 0))?;

    execute!(self.stdout, SetColors(Colors::new(Color::White, Color::DarkBlue)))?;
    write!(self.stdout, "  file.code ")?;

    execute!(self.stdout, SetColors(Colors::new(Color::Black, Color::Grey)))?;
    write!(self.stdout, " x:{} y:{} ", self.cursor.pos.x, self.cursor.pos.y)?;

    execute!(self.stdout, ResetColor)?;

    Ok(())
  }

  pub fn flush(&mut self) -> Result<()> {
    self.stdout.flush()
  }
}
