use crate::{cursor::Cursor, Position, pos};
use crossterm::{
  cursor::MoveTo,
  execute, queue,
  terminal::{size, Clear, ClearType},
  Result,
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
    size()
  }

  pub fn clear(&mut self) -> Result<()> {
    execute!(self.stdout, Clear(ClearType::All))
  }

  pub fn write(&mut self, pos: (u16, u16), text: String) -> Result<()> {
    queue!(self.stdout, MoveTo(pos.0, pos.1))?;
    write!(self.stdout, "{}", text)
  }

  pub fn update_cursor(&mut self) -> Result<()> {
    let x = self.cursor.pos.x - self.pos.x;
    let y = self.cursor.pos.y - self.pos.y;
    queue!(self.stdout, MoveTo(x, y))
  }

  pub fn flush(&mut self) -> Result<()> {
    self.stdout.flush()
  }
}
