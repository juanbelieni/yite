use crossterm::{
  event::{read, Event, KeyCode, KeyModifiers},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  Result,
};
use cursor::MoveDirection;

use std::{cmp::{min, max}, fs::read_to_string};

mod cursor;
mod screen;
use screen::Screen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
  pub x: u16,
  pub y: u16,
}

impl Position {
  pub fn new(x: u16, y: u16) -> Self {
    Self { x, y }
  }
}

#[macro_export]
macro_rules! pos {
  ($x:expr, $y:expr) => {
    Position::new($x, $y)
  };
}

pub struct File {
  pub path: String,
  pub lines: Vec<String>,
}

impl File {
  pub fn new(path: &str) -> Self {
    Self {
      path: path.to_string(),
      lines: Vec::new(),
    }
  }

  pub fn read(&mut self) -> Result<()> {
    let file_content = read_to_string(&self.path)?;
    self.lines = file_content.lines().map(|s| s.to_string()).collect();
    Ok(())
  }
}

fn main() -> Result<()> {
  let mut screen = Screen::new();

  enable_raw_mode()?;
  execute!(screen.stdout, EnterAlternateScreen)?;

  let mut file = File::new("file.code");
  file.read()?;

  loop {
    screen.clear()?;
    let (width, height) = screen.size()?;
    let pos = screen.cursor.pos;

    if pos.x + 2 > screen.pos.x + width {
      screen.pos.x = (pos.x + 2) - width;
    } else if pos.x < screen.pos.x + 1 {
      screen.pos.x = max(0, pos.x as i16 - 1) as u16;
    }

    // if pos.y + 5 > screen.pos.y + height {
    //   screen.pos.y = (pos.y + 5) - height;
    // } else if pos.y < screen.pos.y + 4 {
    //   screen.pos.y = max(0, pos.y as i16 - 4) as u16;
    // }

    if pos.y - screen.pos.y >= height - 5 && screen.pos.y < file.lines.len() as u16 - height {
      screen.pos.y += 1;
    } else if pos.y - screen.pos.y < 5 && screen.pos.y > 0 {
      screen.pos.y -= 1;
    }

    let height_ = min(height, file.lines.len() as u16);


    for y in 0..height_ {
      let y_ = y + screen.pos.y;
      let line: String = file.lines[y_ as usize].chars().skip(screen.pos.x as usize).collect();

      screen.write((0, y), line)?;
    }

    screen.draw_status_bar()?;
    screen.draw_cursor()?;
    screen.flush()?;

    match read()? {
      Event::Key(key_event) => match key_event.code {
        KeyCode::Char('c') | KeyCode::Char('q') if key_event.modifiers == KeyModifiers::CONTROL => {
          disable_raw_mode()?;
          execute!(screen.stdout, LeaveAlternateScreen)?;
          return Ok(());
        }
        KeyCode::Up => screen.cursor.move_cursor(MoveDirection::Up, &file),
        KeyCode::Down => screen.cursor.move_cursor(MoveDirection::Down, &file),
        KeyCode::Left => screen.cursor.move_cursor(MoveDirection::Left, &file),
        KeyCode::Right => screen.cursor.move_cursor(MoveDirection::Right, &file),
        KeyCode::Home => screen.cursor.move_cursor(MoveDirection::Begin, &file),
        KeyCode::End => screen.cursor.move_cursor(MoveDirection::End, &file),
        _ => {}
      },
      _ => {}
    }
  }
}
