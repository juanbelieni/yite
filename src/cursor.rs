use crate::{File, Position};
use std::cmp::min;

pub struct Cursor {
  pub pos: Position,
}

pub enum MoveDirection {
  Up,
  Down,
  Left,
  Right,
  Begin,
  End,
}

impl Cursor {
  pub fn new() -> Self {
    Self {
      pos: Position::new(0, 0),
    }
  }

  pub fn move_cursor(&mut self, direction: MoveDirection, file: &File) {
    match direction {
      MoveDirection::Up => {
        if self.pos.y > 0 {
          self.pos.y -= 1;
          self.pos.x = min(self.pos.x, file.lines[self.pos.y as usize].len() as u16);
        } else {
          self.pos.x = 0;
        }
      }
      MoveDirection::Down => {
        if self.pos.y < file.lines.len() as u16 - 1 {
          self.pos.y += 1;
          self.pos.x = min(self.pos.x, file.lines[self.pos.y as usize].len() as u16);
        } else {
          self.pos.x = file.lines[self.pos.y as usize].len() as u16;
        }
      }
      MoveDirection::Left => {
        if self.pos.x > 0 {
          self.pos.x -= 1;
        } else {
          if self.pos.y > 0 {
            self.pos.y -= 1;
            self.pos.x = file.lines[self.pos.y as usize].len() as u16;
          }
        }
      }
      MoveDirection::Right => {
        if self.pos.x < file.lines[self.pos.y as usize].len() as u16 {
          self.pos.x += 1;
        } else {
          if self.pos.y < file.lines.len() as u16 - 1 {
            self.pos.y += 1;
            self.pos.x = 0;
          }
        }
      }
      MoveDirection::Begin => {
        self.pos.x = 0;
      }
      MoveDirection::End => {
        self.pos.x = file.lines[self.pos.y as usize].len() as u16;
      }
    }
  }
}
